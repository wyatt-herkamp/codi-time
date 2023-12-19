use actix_web::{
    cookie::CookieBuilder, get, http::StatusCode, post, web, web::Data, HttpResponse, Responder,
};
use common::{user_types::Location, Email, Group, IdOrName, PublicUser, User, Username};
use entities::users::{
    does_email_exist, does_username_exist, UserActiveModel, UserModel, UserType,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde::Deserialize;
use tracing::warn;

use super::session::DynSessionManager;
use crate::{
    error::WebsiteError,
    recaptcha::RecaptchaAccess,
    user::{session::SessionManager, Authentication, LoginResponse, NoAuthenticationAllowed},
    utils::password,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(me)
        .service(get_user)
        .service(login)
        .service(register);
}
#[utoipa::path(get,
    impl_for=me,
    path = "/api/me",
    responses(
        (status = 200, description = "You are Logged In", body = User),
        (status = 401, description = "You are not logged in")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/me")]
pub async fn me(auth: Authentication) -> impl Responder {
    HttpResponse::Ok().json(Into::<User>::into(auth))
}

#[utoipa::path(get,
    impl_for=get_user,
    path = "/api/user/{id}",
    responses(
        (status = 200, description = "User", body = PublicUser),
        (status = 401, description = "You are not logged in"),
        (status = 404, description = "User not found")
    ),
)]
#[get("/user/{id}")]
pub async fn get_user(
    database: Data<DatabaseConnection>,
    user_id: web::Path<IdOrName>,
) -> Result<HttpResponse, WebsiteError> {
    let user =
        PublicUser::get_user_by_username_or_id(user_id.into_inner(), database.as_ref()).await?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
#[derive(Debug, Deserialize)]
pub struct Login {
    pub username_or_email: String,
    pub password: String,
    #[serde(rename = "g-recaptcha-response")]
    pub recaptcha: Option<String>,
}

#[post("/login")]
pub async fn login(
    database: Data<DatabaseConnection>,
    login: web::Json<Login>,
    session: Data<DynSessionManager>,
    recaptcha: Data<RecaptchaAccess>,
) -> Result<HttpResponse, WebsiteError> {
    let login = login.into_inner();

    if recaptcha.require_on_login() {
        let Some(recaptcha_result) = login.recaptcha.as_deref() else {
            return Ok(HttpResponse::BadRequest().finish());
        };
        let response = recaptcha
            .verify_response(&recaptcha_result, None)
            .await
            .map_err(|e| {
                warn!("Failed to verify recaptcha: {}", e);
                WebsiteError::RecaptchaError
            })?;
        if !response {
            return Ok(HttpResponse::new(StatusCode::BAD_REQUEST));
        }
    }

    let user = UserModel::get_user_by_username_or_email(login.username_or_email, database.as_ref())
        .await?;
    let Some(user) = user else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    if !password::check_password(&login.password, &user.password)? {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    // Admins get a pass on email verification
    if user.email_verified_at.is_none() && user.group != Group::Admin {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let session = session.create_session(user.id)?;
    let cookie = CookieBuilder::new("session", session.session_id.clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(LoginResponse {
        user: User::from(user),
        session: Some(session),
    }))
}
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    #[serde(default)]
    pub name: String,
    pub username: Username,
    pub email: Email,
    pub password: String,
    pub location: Option<Location>,
    #[serde(rename = "g-recaptcha-response")]
    pub recaptcha: Option<String>,
}

impl RegisterRequest {
    pub fn new_user(self) -> Result<UserActiveModel, WebsiteError> {
        let password = password::encrypt_password(&self.password)?;
        let location = self.location.unwrap_or_default();
        let result = UserActiveModel {
            name: ActiveValue::Set(self.name),
            username: ActiveValue::Set(self.username),
            email: ActiveValue::Set(self.email),
            password: ActiveValue::Set(password),
            location: ActiveValue::Set(location),
            ..Default::default()
        };
        Ok(result)
    }
}

#[post("/register")]
pub async fn register(
    database: Data<DatabaseConnection>,
    signup: web::Json<RegisterRequest>,
    state: Data<crate::State>,
    recaptcha: Data<RecaptchaAccess>,
    _: NoAuthenticationAllowed,
) -> Result<HttpResponse, WebsiteError> {
    let register = signup.into_inner();
    if state.is_first_user() {
        let mut user = register.new_user()?;
        user.group = ActiveValue::Set(Group::Admin);
        let _user = user.save(database.as_ref()).await?;
        state.created_first_user();
        // TODO: Send Email Verification
        return Ok(HttpResponse::NoContent().finish());
    }

    if !state.public_registration {
        return Ok(HttpResponse::Forbidden().finish());
    }

    if recaptcha.require_on_registration() {
        let Some(recaptcha_result) = register.recaptcha.as_deref() else {
            return Ok(HttpResponse::BadRequest().finish());
        };
        let response = recaptcha
            .verify_response(&recaptcha_result, None)
            .await
            .map_err(|e| {
                warn!("Failed to verify recaptcha: {}", e);
                WebsiteError::RecaptchaError
            })?;
        if !response {
            return Ok(HttpResponse::new(StatusCode::BAD_REQUEST));
        }
    }
    if does_email_exist(register.email.clone(), database.as_ref()).await? {
        return Ok(HttpResponse::Conflict().finish());
    }
    if does_username_exist(register.username.clone(), database.as_ref()).await? {
        return Ok(HttpResponse::Conflict().finish());
    }
    let user = register.new_user()?;
    let _user = user.save(database.as_ref()).await?;
    // TODO: Send Email Verification

    Ok(HttpResponse::NoContent().finish())
}
