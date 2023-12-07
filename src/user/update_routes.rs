use actix_web::{put, web, web::Data, HttpResponse};
use chrono::NaiveDate;
use common::{Bio, Email, Preferences, Pronouns, ReportIntervals, User, Username};
use entities::{
    api_keys::{APIKeyColumn, APIKeyEntity},
    users::{
        does_email_exist, does_username_exist, get_password, UserColumn, UserEntity, UserType,
    },
};
use sea_orm::{entity::prelude::*, DatabaseConnection, UpdateResult};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::{
    error::WebsiteError,
    user::Authentication,
    utils::{password, time_utils},
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(update_core)
        .service(update_password)
        .service(update_bio)
        .service(update_report_intervals)
        .service(update_preferences);
}
/// The new account information to update to.
/// All fields are optional.
/// If a field is not provided, it will not be updated.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCore {
    /// The new username to update to.
    pub username: Option<Username>,
    /// The new email to update to.
    pub email: Option<Email>,
    /// The new name to update to.
    pub name: Option<String>,
    /// Whether or not to receive email notifications.
    pub receive_email_notifications: Option<bool>,
    /// Whether or not to show on the leader board.
    pub show_on_leader_board: Option<bool>,
    /// Null to remove location. Otherwise, the new location to update to.
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub location: Option<Option<String>>,
}

impl UpdateCore {
    /// Checks if the update core is empty.
    /// If it is empty, then there is no need to update the user.
    pub fn is_empty(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.name.is_none()
            && self.receive_email_notifications.is_none()
            && self.show_on_leader_board.is_none()
    }
}
#[utoipa::path(put,
    impl_for=update_core,
    path = "/api/me/update/core",
    responses(
        (status = 204, description = "Account Successfully Updated"),
        (status = 400, description = "No fields to update or Invalid fields"),
        (status = 401, description = "You are not logged in."),
        (status = 403, description = "You are logged in with a session."),
        (status = 409, description = "Username or Email already exists.")
    ),
    request_body(content = UpdateCore, description = "The new account information to update to.", content_type = "application/json"),
    security(
        ("session" = [])
    )
)]
#[put("/me/update/core")]
pub async fn update_core(
    auth: Authentication,
    connection: Data<DatabaseConnection>,
    updates: web::Json<UpdateCore>,
) -> Result<HttpResponse, WebsiteError> {
    if !auth.is_session() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    if updates.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No fields to update."));
    }
    let user: User = auth.into();
    let UpdateCore {
        username,
        email,
        name,
        receive_email_notifications,
        show_on_leader_board,
        location,
    } = updates.into_inner();

    let mut update_query = UserEntity::update_many().filter(UserColumn::Id.eq(user.id()));
    let is_email_update = email.is_some();
    if let Some(username) = username {
        if does_username_exist(username.clone(), connection.as_ref()).await? {
            update_query = update_query.col_expr(UserColumn::Username, Expr::value(username));
        } else {
            return Ok(HttpResponse::Conflict().body("Username already exists."));
        }
    }
    if let Some(email) = email {
        if does_email_exist(email.clone(), connection.as_ref()).await? {
            update_query = update_query
                .col_expr(UserColumn::Email, Expr::value(email))
                .col_expr(
                    UserColumn::EmailVerifiedAt,
                    Expr::value(None::<DateTimeWithTimeZone>),
                )
        } else {
            return Ok(HttpResponse::Conflict().body("Email already exists."));
        }
    }
    if let Some(name) = name {
        update_query = update_query.col_expr(UserColumn::Name, Expr::value(name));
    }
    if let Some(receive_email_notifications) = receive_email_notifications {
        update_query = update_query.col_expr(
            UserColumn::ReceiveEmailNotifications,
            Expr::value(receive_email_notifications),
        );
    }
    if let Some(show_on_leader_board) = show_on_leader_board {
        update_query = update_query.col_expr(
            UserColumn::ShowOnLeaderBoard,
            Expr::value(show_on_leader_board),
        );
    }
    if let Some(location) = location {
        // TODO: Location Validation. Should be a Timezone Location. America/New_York, etc.
        update_query = update_query.col_expr(UserColumn::Location, Expr::value(location));
    }
    update_query.exec(connection.as_ref()).await?;

    if is_email_update {
        // TODO: Send email to new email address to confirm. And send email to old email address to notify of change.
    }

    Ok(HttpResponse::NoContent().finish())
}
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UpdatePasswordResponse {
    pub removed_sessions: bool,
    pub removed_api_keys: u64,
}
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePassword {
    pub password: String,
    pub old_password: String,
    #[serde(default)]
    pub force_logout: bool,
    #[serde(default)]
    pub remove_api_keys: bool,
}

#[utoipa::path(put,
    impl_for=update_password,
    path = "/api/me/update/password",
    responses(
        (status = 200, description = "Successfully Updated Password"),
        (status = 400, description = "No fields to update or Invalid fields"),
        (status = 401, description = "You are not logged in."),
        (status = 403, description = "You are logged in with a session or Old Password is incorrect."),
    ),
    request_body(content = UpdatePassword, description = "The new account information to update to.", content_type = "application/json"),
    security(
        ("session" = [])
    )
)]
#[put("/me/update/password")]
pub async fn update_password(
    auth: Authentication,
    connection: Data<DatabaseConnection>,
    updates: web::Json<UpdatePassword>,
) -> Result<HttpResponse, WebsiteError> {
    if !auth.is_session() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let user: User = auth.into();
    let Some(password_in_db) = get_password(user.id, connection.as_ref()).await? else {
        error!(
            "User {} was not found. Directly After finding User???",
            user.id()
        );
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let UpdatePassword {
        password,
        old_password,
        force_logout,
        remove_api_keys,
    } = updates.into_inner();
    if !password::check_password(&old_password, &password_in_db)? {
        return Ok(HttpResponse::Forbidden().body("Old Password is incorrect."));
    }
    UserEntity::update_many()
        .filter(UserColumn::Id.eq(user.id()))
        .col_expr(UserColumn::Password, Expr::value(password))
        .col_expr(UserColumn::RequirePasswordChange, Expr::value(false))
        .col_expr(
            UserColumn::PasswordChangedAt,
            Expr::value(time_utils::get_current_time()),
        )
        .exec(connection.as_ref())
        .await?;
    // TODO Send Email to notify of password change.

    let mut response = UpdatePasswordResponse::default();
    if force_logout {
        // TODO: Remove all sessions for user. Change below to true
        response.removed_sessions = false;
    }
    if remove_api_keys {
        let UpdateResult { rows_affected } = APIKeyEntity::update_many()
            .filter(
                APIKeyColumn::UserId
                    .eq(user.id())
                    .and(APIKeyColumn::Revoked.eq(None::<DateTimeWithTimeZone>)),
            )
            .col_expr(
                APIKeyColumn::Revoked,
                Expr::value(Some(time_utils::get_current_time())),
            )
            .exec(connection.as_ref())
            .await?;
        info!("Removed {rows_affected} API Keys");
        response.removed_api_keys = rows_affected;
    }
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBio {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub pronouns: Option<Option<Pronouns>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub location: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub bio: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub birthday: Option<Option<NaiveDate>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub website: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub discord: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub github: Option<Option<String>>,
}
impl UpdateBio {
    pub fn apply_to_bio(self, bio_to_update: &mut Bio) {
        if let Some(pronouns) = self.pronouns {
            bio_to_update.pronouns = pronouns;
        }
        if let Some(location) = self.location {
            bio_to_update.location = location;
        }
        if let Some(bio) = self.bio {
            bio_to_update.bio = bio;
        }
        if let Some(birthday) = self.birthday {
            bio_to_update.birthday = birthday;
        }
        if let Some(website) = self.website {
            bio_to_update.website = website;
        }
        if let Some(discord) = self.discord {
            bio_to_update.discord = discord;
        }
        if let Some(github) = self.github {
            bio_to_update.github = github;
        }
    }
    pub fn is_empty(&self) -> bool {
        self.pronouns.is_none()
            && self.location.is_none()
            && self.bio.is_none()
            && self.birthday.is_none()
            && self.website.is_none()
            && self.discord.is_none()
            && self.github.is_none()
    }
}
#[utoipa::path(put,
    impl_for=update_bio,
    path = "/api/me/update/bio",
    responses(
        (status = 204, description = "Successfully Updated Bio"),
        (status = 400, description = "No fields to update or Invalid fields"),
        (status = 401, description = "You are not logged in."),
        (status = 403, description = "You are logged in with a session."),
    ),
    request_body(content = Bio, description = "The new account information to update to.", content_type = "application/json"),
    security(
        ("session" = [])
    )
)]
#[put("/me/update/bio")]
pub async fn update_bio(
    auth: Authentication,
    connection: Data<DatabaseConnection>,
    updates: web::Json<UpdateBio>,
) -> Result<HttpResponse, WebsiteError> {
    if !auth.is_session() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let mut user: User = auth.into();
    let updates = updates.into_inner();
    if updates.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No fields to update."));
    }
    updates.apply_to_bio(&mut user.bio);
    UserEntity::update_many()
        .filter(UserColumn::Id.eq(user.id()))
        .col_expr(UserColumn::Bio, Expr::value(user.bio))
        .exec(connection.as_ref())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(put,
    impl_for=update_report_intervals,
    path = "/api/me/update/report-intervals",
    responses(
        (status = 204, description = "Successfully Updated Report Intervals"),
        (status = 400, description = "No fields to update or Invalid fields"),
        (status = 401, description = "You are not logged in."),
    ),
    request_body(content = Vec<ReportIntervals>, description = "The new account information to update to.", content_type = "application/json"),
    security(
        ("api_key" = []),
        ("session" = [])
    )
)]
#[put("/me/update/report-intervals")]
pub async fn update_report_intervals(
    auth: Authentication,
    connection: Data<DatabaseConnection>,
    updates: web::Json<Vec<ReportIntervals>>,
) -> Result<HttpResponse, WebsiteError> {
    let user: User = auth.into();
    let updates = updates.into_inner();
    UserEntity::update_many()
        .filter(UserColumn::Id.eq(user.id()))
        .col_expr(UserColumn::ReportInterval, Expr::value(updates))
        .exec(connection.as_ref())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct UpdatePreferences {
    pub share_editors: Option<bool>,
    pub share_operating_systems: Option<bool>,
    pub share_languages: Option<bool>,
    pub share_labels: Option<bool>,
    pub share_projects: Option<bool>,
}
impl UpdatePreferences {
    pub fn update_preferences(self, preferences_to_update: &mut Preferences) -> bool {
        let mut update_occurred = false;
        if let Some(share_editors) = self.share_editors {
            if preferences_to_update.share_editors != share_editors {
                update_occurred = true;
            }
            preferences_to_update.share_editors = share_editors;
        }
        if let Some(share_operating_systems) = self.share_operating_systems {
            if preferences_to_update.share_operating_systems != share_operating_systems {
                update_occurred = true;
            }
            preferences_to_update.share_operating_systems = share_operating_systems;
        }
        if let Some(share_languages) = self.share_languages {
            if preferences_to_update.share_languages != share_languages {
                update_occurred = true;
            }
            preferences_to_update.share_languages = share_languages;
        }
        if let Some(share_labels) = self.share_labels {
            if preferences_to_update.share_labels != share_labels {
                update_occurred = true;
            }
            preferences_to_update.share_labels = share_labels;
        }
        if let Some(share_projects) = self.share_projects {
            if preferences_to_update.share_projects != share_projects {
                update_occurred = true;
            }
            preferences_to_update.share_projects = share_projects;
        }
        update_occurred
    }
}
#[utoipa::path(put,
    impl_for=update_preferences,
    path = "/api/me/update/preferences",
    responses(
        (status = 204, description = "Successfully Updated Report Intervals"),
        (status = 400, description = "No fields to update or Invalid fields"),
        (status = 401, description = "You are not logged in."),
        (status = 403, description = "You are logged in with a session."),
    ),
    request_body(content = UpdatePreferences, description = "The new account information to update to.", content_type = "application/json"),
    security(
        ("api_key" = [])
    )
)]
#[put("/me/update/preferences")]
pub async fn update_preferences(
    auth: Authentication,
    connection: Data<DatabaseConnection>,
    updates: web::Json<UpdatePreferences>,
) -> Result<HttpResponse, WebsiteError> {
    let mut user: User = auth.into();
    let updates = updates.into_inner();
    if !updates.update_preferences(&mut user.preferences) {
        return Ok(HttpResponse::BadRequest().body("No fields to update."));
    }
    UserEntity::update_many()
        .filter(UserColumn::Id.eq(user.id()))
        .col_expr(UserColumn::Preferences, Expr::value(user.preferences))
        .exec(connection.as_ref())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
