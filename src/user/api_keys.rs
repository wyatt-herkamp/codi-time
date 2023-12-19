use actix_web::{get, post, web, web::Data, HttpRequest, HttpResponse};
use common::user_types::api_token::FromCLI;
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::routes::Login;
use crate::{cli_access::CLIAccess, error::WebsiteError, state::State};

pub fn init(cfg: &mut web::ServiceConfig) {}
