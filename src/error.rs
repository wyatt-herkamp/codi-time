use actix_web::ResponseError;
use either::Either;
use sea_orm::DbErr;
use serde::{ser::SerializeStruct, Serialize};
use simdutf8::basic::Utf8Error;
use this_actix_error::ActixError;
use thiserror::Error;

use crate::user::session::SessionError;

#[derive(Debug, Error, ActixError)]
pub enum WebsiteError {
    #[error("IO Error")]
    #[status_code(INTERNAL_SERVER_ERROR)]
    IoError(#[from] std::io::Error),
    #[error("Database Error")]
    DatabaseError(Either<DbErr, sqlx::Error>),
    #[error("Argon2 Error")]
    #[status_code(BAD_REQUEST)]
    EncryptPasswordError(argon2::password_hash::Error),
    #[error("Unauthorized")]
    #[status_code(UNAUTHORIZED)]
    Unauthorized,
    #[error("Forbidden")]
    #[status_code(FORBIDDEN)]
    Forbidden,
    #[error("Session Error")]
    #[status_code(INTERNAL_SERVER_ERROR)]
    SessionError(#[from] SessionError),
    #[error("Not Found")]
    #[status_code(NOT_FOUND)]
    NotFound,
    #[error("Bad Request")]
    #[status_code(BAD_REQUEST)]
    UTF8ERROR(Utf8Error),
    #[error("Exceeds Maximum Length")]
    #[status_code(BAD_REQUEST)]
    ExceedsMaxLength,
    #[error("You Are Not Allowed to be Logged In")]
    #[status_code(UNAUTHORIZED)]
    NotAllowedToBeLoggedIn,
    #[error("Internal Server Error: Recaptcha Error")]
    #[status_code(INTERNAL_SERVER_ERROR)]
    RecaptchaError,
}

/// Implemented for responses that can partially fail.
impl Serialize for WebsiteError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("WebsiteError", 2)?;
        s.serialize_field("status_code", &self.status_code().to_string())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

impl From<DbErr> for WebsiteError {
    fn from(error: DbErr) -> Self {
        Self::DatabaseError(Either::Left(error))
    }
}
impl From<sqlx::Error> for WebsiteError {
    fn from(error: sqlx::Error) -> Self {
        Self::DatabaseError(Either::Right(error))
    }
}
