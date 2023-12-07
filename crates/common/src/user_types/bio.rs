use std::{fmt::Display, str::FromStr};

use chrono::NaiveDate;
use digestible::Digestible;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumMessage, EnumString};
use utoipa::ToSchema;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    ToSchema,
    Serialize,
    Deserialize,
    Digestible,
)]
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    #[strum(default)]
    #[serde(rename = "other")]
    Other(String),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, ToSchema, Digestible,
)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromJsonQueryResult))]
#[serde(default)]
pub struct Bio {
    pub pronouns: Option<Pronouns>,
    pub location: Option<String>,
    pub bio: Option<String>,
    #[digestible(digest_with = digest_with_hash)]
    pub birthday: Option<NaiveDate>,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub github: Option<String>,
}
