use std::str::FromStr;

#[cfg(feature = "sea-orm")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, EnumIter)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
#[strum(serialize_all = "lowercase")]
pub enum HeartbeatType {
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "file"))]
    File,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "app"))]
    App,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "domain"))]
    Domain,
}

impl Serialize for HeartbeatType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for HeartbeatType {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let heart_beat_type = String::deserialize(deserializer)?;
        HeartbeatType::from_str(&heart_beat_type).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, EnumIter)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
#[strum(serialize_all = "lowercase")]
pub enum HeartbeatCategory {
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "coding"))]
    Coding,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "building"))]
    Building,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "indexing"))]
    Indexing,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "debugging"))]
    Debugging,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "browsing"))]
    Browsing,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "running tests"))]
    #[strum(serialize = "running tests")]
    RunningTests,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "writing tests"))]
    #[strum(serialize = "writing tests")]
    WritingTests,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "manual testing"))]
    #[strum(serialize = "manual testing")]
    ManualTesting,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "writing docs"))]
    #[strum(serialize = "writing docs")]
    WritingDocs,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "code reviewing"))]
    #[strum(serialize = "code review")]
    CodeReview,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "communicating"))]
    Communicating,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "researching"))]
    Researching,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "learning"))]
    Learning,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "designing"))]
    Designing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromJsonQueryResult))]
pub struct CodeChanges {
    pub lines_added: u32,
    pub lines_removed: u32,
}
