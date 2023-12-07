//! Routes to Build Compatibility with the WakaTime API
//!  In the Future, We will have our own API, but for now, we will use the WakaTime API

use actix_web::{
    cookie::CookieBuilder, get, http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::user::Authentication;
/// Base Route /api/waka-time
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(heartbeat);
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct WakaTimeHeartbeat {
    pub entity: String,
    pub category: String,
    pub created_at: i64,
    pub editor: String,
    pub language: String,
}
#[post("/api/waka-time/heartbeat")]
pub async fn heartbeat(auth: Authentication, request: HttpRequest) -> HttpResponse {
    todo!()
}
