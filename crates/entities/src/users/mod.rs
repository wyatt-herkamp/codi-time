pub mod pub_user;

mod utils;
use common::{
    database_helpers::{BasicTableTrait, HasNameColumn},
    user_types::{
        bio::Bio, group::Group, preferences::Preferences, report_intervals::ReportIntervals, Email,
        Location, Username,
    },
};
use helper_macros::DatabaseHelpers;
use sea_orm::entity::prelude::*;
use sea_orm_exports::SeaORMExports;
use serde::Serialize;
use strum::EnumIter;
pub use utils::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, SeaORMExports, DatabaseHelpers)]
#[exports(User, has_relation)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[column(id)]
    pub id: i64,
    #[sea_orm(default_value = "")]
    pub name: String,
    #[column(name)]
    pub username: Username,
    #[sea_orm(default_expr = "Bio::default()")]
    pub bio: Bio,
    pub email: Email,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub group: Group,
    pub receive_email_notifications: bool,
    #[serde(skip_serializing)]
    pub password: String,
    pub require_password_change: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub password_changed_at: DateTimeWithTimeZone,
    #[sea_orm(default_value = "Etc/UTC")]
    pub location: Location,
    #[sea_orm(default_value = "true")]
    pub show_on_leader_board: bool,
    #[sea_orm(default_value = "{}")]
    pub report_interval: Vec<ReportIntervals>,
    #[sea_orm(default_expr = "Preferences::default()")]
    pub preferences: Preferences,
    #[sea_orm(default_value = "false")]
    pub banned: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_logged_in: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    #[column(created)]
    pub created: DateTimeWithTimeZone,
}
impl ActiveModelBehavior for ActiveModel {}

// Foreign Key group_id to Group::id

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::connections::Entity")]
    Connections,
    #[sea_orm(has_many = "crate::heartbeats::Entity")]
    Heartbeats,
    #[sea_orm(has_many = "crate::projects::Entity")]
    Projects,
    #[sea_orm(has_many = "crate::api_keys::Entity")]
    APIKeys,
    #[sea_orm(has_many = "crate::teams::team_members::Entity")]
    TeamMembers,
    #[sea_orm(has_many = "crate::custom_languages::languages::Entity")]
    CustomLanguages,
    #[sea_orm(has_many = "crate::custom_languages::categories::Entity")]
    CustomLanguageCategories,
}

impl Related<crate::connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connections.def()
    }
}

impl Related<crate::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}
impl Related<crate::heartbeats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Heartbeats.def()
    }
}
impl Related<crate::api_keys::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::APIKeys.def()
    }
}

impl Related<crate::teams::team_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamMembers.def()
    }
}
impl Related<crate::custom_languages::languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomLanguages.def()
    }
}
impl Related<crate::custom_languages::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomLanguageCategories.def()
    }
}
