pub mod config;
pub mod error;
pub mod open_api;
pub mod state;
pub mod tracing_setup;
pub mod utils;
use std::sync::atomic::AtomicBool;
pub use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};
pub mod projects;
pub mod recaptcha;
pub mod user;
pub mod waka_time;
use actix_cors::Cors;
use actix_web::{get, web::Data, App, HttpServer, Scope};
use clap::Parser;
use config::{ServerConfig, SessionConfigFull};
use entities::users::does_first_user_exist;
use migration::{Migrator, MigratorTrait};
use open_api::ApiDoc;
use recaptcha::RecaptchaAccess;
use rustls::{Certificate, PrivateKey, ServerConfig as RustlsServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use sea_orm::Database;
pub mod cli_access;
use human_panic::setup_panic;
use state::State;
use tracing_actix_web::TracingLogger;
use user::{
    middleware::HandleSession,
    session::{SessionManager, SessionManagerType},
};
use utoipa::{openapi::OpenApi as Docs, OpenApi};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "config.toml")]
    config: PathBuf,
    /// Rewrites the config filling in any missing values with the default values. Good for updating the config during development
    #[clap(long)]
    rewrite_config: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_panic!();

    let args: Args = Args::parse();
    let ServerConfig {
        bind_address,
        home_url,
        workers,
        tls,
        database,
        session,
        tracing,
        public_registration,
        recaptcha,
    } = if !args.config.exists() {
        let config = ServerConfig::default();
        let config = toml::to_string(&config)
            .expect("Failed to serialize config. Please report this as a bug.");
        std::fs::write(&args.config, config).expect(&format!("Failed to write config file. Please ensure that the server has write permissions to {:?}",args.config));
        return Ok(());
    } else {
        let config = std::fs::read_to_string(&args.config).unwrap();
        let config: ServerConfig = toml::from_str(&config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        if args.rewrite_config {
            let config = toml::to_string(&config)
                .expect("Failed to serialize config. Please report this as a bug.");
            std::fs::write(&args.config, config).expect(&format!("Failed to write config file. Please ensure that the server has write permissions to {:?}",args.config));
        }
        config
    };
    tracing_setup::setup(tracing).expect("Failed to setup tracing");
    let SessionConfigFull {
        manager,
        session_config,
    } = session;
    let database = Database::connect(database)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Migrator::up(&database, None)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let recaptcha_access = RecaptchaAccess::new(recaptcha)
        .map(Data::new)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create recaptcha access: {}", e),
            )
        })?;
    let first_user = does_first_user_exist(&database)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let state = Data::new(State {
        is_first_user: AtomicBool::new(!first_user),
        home_url,
        public_registration,
        recaptcha_config: recaptcha_access.state_value(),
        ..Default::default()
    });
    let session = SessionManagerType::new(manager, session_config.clone()).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create session manager: {}", e),
        )
    })?;
    let database = Data::new(database);
    let session = Data::new(session);
    let cli_access = Data::new(cli_access::CLIAccess::new());
    let openapi = Data::new(ApiDoc::openapi());

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .app_data(database.clone())
            .app_data(session.clone())
            .app_data(state.clone())
            .app_data(recaptcha_access.clone())
            .app_data(openapi.clone())
            .app_data(cli_access.clone())
            .wrap(TracingLogger::default())
            .wrap(cors)
            .service(openapi_json)
            .service(get_state)
            .service(
                Scope::new("/api")
                    .wrap(HandleSession {
                        session_manager: session.clone().into_inner(),
                    })
                    .configure(user::routes::init)
                    .configure(user::update_routes::init)
                    .configure(user::cli::init)
                    .configure(projects::init)
                    .service(Scope::new("/admin")),
            )
    });
    let server = if let Some(workers) = workers {
        server.workers(workers)
    } else {
        server
    };

    let server = if let Some(tls) = tls {
        let mut cert_file = BufReader::new(File::open(tls.certificate_chain)?);
        let mut key_file = BufReader::new(File::open(tls.private_key)?);

        let cert_chain = certs(&mut cert_file)
            .expect("server certificate file error")
            .into_iter()
            .map(Certificate)
            .collect();
        let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_file)
            .expect("server private key file error")
            .into_iter()
            .map(PrivateKey)
            .collect();

        let config = RustlsServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, keys.remove(0))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        server.bind_rustls_021(bind_address, config)?
    } else {
        server.bind(bind_address)?
    };
    server.run().await?;
    Ok(())
}

#[get("/api/docs/openapi.json")]
async fn openapi_json(openapi: Data<Docs>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(openapi.as_ref())
}
#[utoipa::path(
    get,
    impl_for = get_state,
    path = "/api/state",
    responses(
        (status = 200, description = "State", body = State),
    ),
)]
#[get("/api/state")]
async fn get_state(state: Data<State>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(state.as_ref())
}
