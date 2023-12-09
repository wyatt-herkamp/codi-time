use actix_web::{
    cookie::CookieBuilder, get, http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse,
    Responder,
};
use common::{user_types::api_token::FromCLI, Email, Group, PublicUser, User, Username};
use either::Either;
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::routes::Login;
use crate::{cli_access::CLIAccess, error::WebsiteError, state::State};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(init_session)
        .service(retrieve_result)
        .service(complete_access);
}
#[derive(Debug, Deserialize)]
pub struct NewCLIRequest {
    pub username: Option<String>,
    #[serde(flatten)]
    pub from_cli: FromCLI,
}
#[derive(Debug, Serialize)]
pub struct InitSessionResponse {
    pub token: String,
    pub absolute_url: Option<String>,
}
#[utoipa::path(post,
    impl_for=init_session,
    path = "/api/cli/init-session",
    request_body(content = NewCLIRequest, description = "New Access Request", content_type = "application/json"),
    responses(
        (status = 200, description = "CLI Access was Initiated", body = InitSessionResponse),
        (status = 401, description = "IP Address was blacklisted"),
        (status = 400, description = "NO IP Address was provided")
    )
)]
#[post("/init-session")]
pub async fn init_session(
    body: web::Json<NewCLIRequest>,
    state: Data<State>,
    cli_access: Data<CLIAccess>,
    request: HttpRequest,
) -> Result<HttpResponse, WebsiteError> {
    let body = body.into_inner();

    let ip_address = if let Some(ip) = request.connection_info().realip_remote_addr() {
        ip.to_owned()
    } else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let key = cli_access.create_new_pending_access(body.from_cli, body.username, ip_address);
    let absolute_url = if let Some(state) = state.home_url.as_ref() {
        Some(format!("{}/login-cli/{}", state, key))
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(InitSessionResponse {
        token: key,
        absolute_url,
    }))
}

#[utoipa::path(get,
    impl_for=retrieve_result,
    path = "/api/retrieve-result/{key}",
    responses(
        (status = 200, description = "CLI Access was Initiated", body = APIToken),
        (status = 401, description = "IP Address was blacklisted"),
    )
)]
#[get("/retrieve-result/{key}")]
pub async fn retrieve_result(
    path: web::Path<String>,
    cli_access: Data<CLIAccess>,
    request: HttpRequest,
) -> Result<HttpResponse, WebsiteError> {
    let ip_address = if let Some(ip) = request.connection_info().realip_remote_addr() {
        ip.to_owned()
    } else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let key = path.into_inner();

    let Some(cli_access) = cli_access.get_unclaimed_access(&key) else {
        return Ok(HttpResponse::Processing().finish());
    };
    if ip_address != cli_access.ip_address {
        warn!(
            "IP Address Mismatch: {} != {}",
            ip_address, cli_access.ip_address
        );
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let api_token = cli_access.api_token;

    Ok(HttpResponse::Ok().json(api_token))
}

#[post("/complete-access/{key}")]
pub async fn complete_access(
    path: web::Path<String>,
    login: web::Json<Login>,
    cli_access: Data<CLIAccess>,
    request: HttpRequest,
) -> Result<HttpResponse, WebsiteError> {
    
    todo!();
}
