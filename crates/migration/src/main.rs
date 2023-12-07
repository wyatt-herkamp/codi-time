use migration::Migrator;
use sea_orm_migration::{
    sea_orm::{self, ConnectOptions},
    MigratorTrait,
};
use tracing::Level;
use tracing_subscriber;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .init();

    let env = option_env!("POSTGRES_INSTANCE");

    let url = if let Some(env) = env {
        println!("POSTGRES_INSTANCE set to {}", env);
        if env.ends_with("/") {
            format!("{env}test_codi_time_db")
        } else {
            format!("{env}/test_codi_time_db")
        }
    } else {
        println!("POSTGRES_INSTANCE not set, skipping test");
        return;
    };
    println!("Connecting to {:?}", url);
    let database_connection = sea_orm::Database::connect(ConnectOptions::new(url))
        .await
        .expect("Failed to connect to database");

    Migrator::up(&database_connection, None)
        .await
        .expect("Failed to run migrations");
}
