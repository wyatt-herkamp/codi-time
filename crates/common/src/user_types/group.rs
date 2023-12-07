use digestible::Digestible;
#[cfg(feature = "sea-orm")]
use sea_orm::entity::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Deserialize,
    Serialize,
    EnumString,
    Display,
    EnumIter,
    ToSchema,
    Digestible,
)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum Group {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Moderator")]
    Moderator,
    /// # Permissions
    /// - View Blog Drafts
    /// - See More Details
    #[sea_orm(string_value = "User")]
    #[default]
    User,
}
