pub mod cli;
pub mod middleware;
pub mod routes;
pub mod session;
pub mod update_routes;
use std::fmt::Debug;

use actix_web::{dev::Payload, web::Data, FromRequest, HttpMessage, HttpRequest};
use common::{APIToken, User};
use derive_more::{AsRef, From, Into};
use digestible::Digestible;
use either::Either;
use entities::users::UserType;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use sea_orm::{DatabaseConnection, DbErr};
use serde::Serialize;
use strum::EnumIs;
use this_actix_error::ActixError;
use thiserror::Error;
use tracing::{instrument, warn};
use utoipa::ToSchema;

use crate::{error::WebsiteError, user::session::Session, utils};

#[derive(Serialize, Digestible, Debug, ToSchema)]
pub struct LoginResponse {
    user: User,
    #[digestible(skip)]
    session: Option<Session>,
}
#[derive(Debug, Error, ActixError)]
pub enum AuthenticationError {
    #[status_code(UNAUTHORIZED)]
    #[error("Unauthorized")]
    NoAuthenticationProvided,
    #[status_code(UNAUTHORIZED)]
    #[error("Invalid API Key")]
    InvalidAPIKey,
    #[status_code(UNAUTHORIZED)]
    #[error("Invalid Session")]
    InvalidSession,
    #[status_code(FORBIDDEN)]
    #[error("Must be a session")]
    MustBeSession,
    #[error("Database Error")]
    #[status_code(INTERNAL_SERVER_ERROR)]
    DatabaseError(Either<DbErr, sqlx::Error>),
}
impl From<DbErr> for AuthenticationError {
    fn from(error: DbErr) -> Self {
        Self::DatabaseError(Either::Left(error))
    }
}
impl From<sqlx::Error> for AuthenticationError {
    fn from(error: sqlx::Error) -> Self {
        Self::DatabaseError(Either::Right(error))
    }
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
    ) -> Result<Authentication, AuthenticationError> {
        let result = match raw {
            AuthenticationRaw::Session(session) => {
                User::get_by_id(database.as_ref(), session.user_id)
                    .await?
                    .map(|user| Authentication::Session { user, session })
                    .ok_or(AuthenticationError::InvalidSession)
            }
            AuthenticationRaw::APIToken(token) => {
                let as_sha256 = utils::sha256::encode_to_string(&token);
                entities::api_keys::get_user_and_token(&as_sha256, database.as_ref())
                    .await?
                    .map(|(token, user)| Authentication::APIToken { user, token })
                    .ok_or(AuthenticationError::InvalidAPIKey)
            }
        };
        result
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
    type Error = AuthenticationError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    /// Extracts the authentication data from the request.
    #[instrument(skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let raw_auth = req.extensions_mut().get::<AuthenticationRaw>().cloned();
        let Some(raw_auth) = raw_auth else {
            return Box::pin(async move { Err(AuthenticationError::NoAuthenticationProvided) });
        };
        let database = req
            .app_data::<Data<DatabaseConnection>>()
            .expect("Unable to get Database Ref")
            .clone();
        return Box::pin(async move { Authentication::new(database, raw_auth).await });
    }
}
#[derive(Debug, Clone, AsRef, Into, From)]
pub struct SessionAuthentication {
    #[as_ref]
    #[into]
    pub user: User,
    #[as_ref]
    #[into]
    pub session: Session,
}
impl SessionAuthentication {
    pub async fn new(
        session: Session,
        database_connection: Data<DatabaseConnection>,
    ) -> Result<Self, AuthenticationError> {
        let user = User::get_by_id(database_connection.as_ref(), session.user_id)
            .await?
            .ok_or_else(|| {
                warn!(
                    "Session {} has invalid user id {}",
                    session.session_id, session.user_id
                );
                AuthenticationError::InvalidSession
            })?;
        Ok(Self { user, session })
    }
}
impl FromRequest for SessionAuthentication {
    type Error = AuthenticationError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    /// Extracts the authentication data from the request.
    #[instrument(skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let raw_auth = req.extensions_mut().get::<AuthenticationRaw>().cloned();
        let Some(raw_auth) = raw_auth else {
            return Box::pin(async move { Err(AuthenticationError::NoAuthenticationProvided) });
        };
        let AuthenticationRaw::Session(session) = raw_auth else {
            return Box::pin(async move { Err(AuthenticationError::MustBeSession) });
        };
        let database = req
            .app_data::<Data<DatabaseConnection>>()
            .expect("Unable to get Database Ref")
            .clone();
        return Box::pin(async move { SessionAuthentication::new(session, database).await });
    }
}
