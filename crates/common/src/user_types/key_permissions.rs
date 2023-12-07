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
pub enum APIKeyPermissions {
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
