use digestible::Digestible;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    Digestible,
)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromJsonQueryResult))]
#[serde(default)]
pub struct Preferences {
    pub share_editors: bool,
    pub share_operating_systems: bool,
    pub share_languages: bool,
    pub share_labels: bool,
    pub share_projects: bool,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            share_editors: true,
            share_operating_systems: true,
            share_languages: true,
            share_labels: true,
            share_projects: true,
        }
    }
}
