use digestible::Digestible;
#[cfg(feature = "sea-orm")]
use sea_orm::entity::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

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
    Digestible,
)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum ReportIntervals {
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Daily"))]
    Daily,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Weekly"))]
    Weekly,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Monthly"))]
    Monthly,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Quarterly"))]
    Quarterly,
    /// Processes on the 1st of January
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Yearly"))]
    Yearly,
}
