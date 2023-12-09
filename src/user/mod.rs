pub mod cli;
pub mod middleware;
pub mod routes;
pub mod session;
pub mod update_routes;
use std::{fmt::Debug, fs::Permissions};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpMessage, HttpRequest};
use common::{APIToken, User};
use digestible::Digestible;
use entities::{api_keys::APIKeyModel, users::UserType};
use futures_util::{
    future::{ready, LocalBoxFuture, Ready},
    ready,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_with::As;
use strum::EnumIs;
use tracing::{instrument, Span};
use utoipa::ToSchema;

use crate::{error::WebsiteError, user::session::Session};

#[derive(Serialize, Digestible, Debug, ToSchema)]
pub struct LoginResponse {
    user: User,
    #[digestible(skip)]
    session: Option<Session>,
}

/// The raw authentication data.
/// Pulled from the middleware.
/// Will be converted to an [Authentication] type.
#[derive(Debug, Clone, EnumIs)]
pub enum AuthenticationRaw {
    Session(Session),
    APIToken(String),
}

/// The authorized user.
/// Containing the user model and any additional data to the authentication method.

#[derive(Debug, Clone, EnumIs)]
pub enum Authentication {
    Session { user: User, session: Session },
    APIToken { user: User, token: APIToken },
}
impl Into<User> for Authentication {
    fn into(self) -> User {
        match self {
            Authentication::Session { user, .. } => user,
            Authentication::APIToken { user, .. } => user,
        }
    }
}
impl AsRef<User> for Authentication {
    fn as_ref(&self) -> &User {
        match self {
            Authentication::Session { user, .. } => user,
            Authentication::APIToken { user, .. } => user,
        }
    }
}
impl Authentication {
    #[instrument(skip(database, raw))]
    pub async fn new(
        database: Data<DatabaseConnection>,
        raw: AuthenticationRaw,
    ) -> Result<Option<Authentication>, WebsiteError> {
        let result = match raw {
            AuthenticationRaw::Session(session) => {
                User::get_by_id(database.as_ref(), session.user_id)
                    .await?
                    .map(|user| Authentication::Session { user, session })
            }
            AuthenticationRaw::APIToken(token) => {
                entities::api_keys::utils::get_user_and_token(&token, database.as_ref())
                    .await?
                    .map(|(token, user)| Authentication::APIToken { user, token })
            }
        };
        Ok(result)
    }
    /// Copies the id from the UserModel.
    pub fn id(&self) -> i64 {
        match self {
            Authentication::Session { user, .. } => user.id,
            Authentication::APIToken { user, .. } => user.id,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoAuthenticationAllowed;

impl FromRequest for NoAuthenticationAllowed {
    type Error = WebsiteError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if req.extensions().get::<AuthenticationRaw>().is_some() {
            return ready(Err(WebsiteError::NotAllowedToBeLoggedIn));
        }
        ready(Ok(NoAuthenticationAllowed))
    }
}
impl FromRequest for Authentication {
    type Error = WebsiteError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    /// Extracts the authentication data from the request.
    #[instrument(skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let model = req.extensions_mut().get::<AuthenticationRaw>().cloned();
        Span::current().record("auth", &format!("{:?}", model.as_ref()));
        if let Some(model) = model {
            let database = req
                .app_data::<Data<DatabaseConnection>>()
                .expect("Unable to get Database Ref")
                .clone();
            return Box::pin(async move {
                let model = Authentication::new(database, model).await?;
                if let Some(model) = model {
                    return Ok(model);
                }
                Err(WebsiteError::Unauthorized)
            });
        }
        Box::pin(async move { Err(WebsiteError::Unauthorized) })
    }
}
