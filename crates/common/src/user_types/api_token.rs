use digestible::Digestible;
#[cfg(feature = "sea-orm")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;
/// Permissions/Scopes for API Keys
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum APITokenPermissions {
    /// Push Heartbeat
    ///
    /// # Note
    /// Required for the the editor plugins to work
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "WriteHeartbeat"))]
    WriteHeartbeat,
    /// Pull Heartbeat
    ///
    /// # Note
    /// Required for the the editor plugins to work
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "ReadHeartbeat"))]
    ReadHeartbeat,
    /// Read Usage
    ///
    /// # Note
    /// Required for the the editor plugins to work
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "ReadUsage"))]
    ReadUsage,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema, Digestible)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromJsonQueryResult))]
#[serde(default)]
pub struct FromCLI {
    pub machine_hostname: String,
    pub cli_version: String,
    pub cli_platform: String,
    pub cli_commit: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromQueryResult))]

pub struct APIToken {
    pub id: i64,
    /// Has One relation to users::id
    pub user_id: i64,
    pub permissions: Vec<APITokenPermissions>,
    pub from_cli: Option<FromCLI>,
    /// Key is invalid. Invalid keys are kept for a short period for warning and logging purposes.
    pub revoked: Option<DateTimeWithTimeZone>,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub created: DateTimeWithTimeZone,
}
