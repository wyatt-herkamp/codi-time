///
/// Categories and Languages are initialized by the data in the `languages` folder
///
/// A User can add a new Language or Category that is only available to them
/// A Team can add a new Language or Category that is available to all members of the Team
///
/// An Admin can add a new Language or Category that is available to all Users
use sea_orm::DeriveActiveEnum;

pub mod categories;
pub mod languages;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
#[derive(
    DeriveActiveEnum,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deserialize,
    Serialize,
    EnumString,
    Display,
    EnumIter,
)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Source {
    /// From Language Definition Files
    #[sea_orm(string_value = "FromDefault")]
    FromDefault,
    /// From Language Definition Files then were modified by an Admin
    #[sea_orm(string_value = "ModifiedDefault")]
    ModifiedDefault,
    /// Added by a User for their own use
    #[sea_orm(string_value = "FromUser")]
    FromUser,
    /// Added by a Team for their own use
    #[sea_orm(string_value = "FromTeam")]
    FromTeam,
    /// Public Definitions Added by Admin
    #[sea_orm(string_value = "FromAdmin")]
    FromAdmin,
}
