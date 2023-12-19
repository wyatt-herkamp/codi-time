/// Public User Types
use chrono::{DateTime, FixedOffset};
use digestible::Digestible;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    bio::Bio, group::Group, preferences::Preferences, report_intervals::ReportIntervals, Email,
    Location, Username,
};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema, Digestible)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromQueryResult))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: Username,
    pub bio: Bio,
    pub email: Email,
    #[digestible(digest_with = digest_with_hash)]
    pub email_verified_at: Option<DateTime<FixedOffset>>,
    pub group: Group,
    pub receive_email_notifications: bool,
    pub require_password_change: bool,
    #[digestible(digest_with = digest_with_hash)]
    pub password_changed_at: DateTime<FixedOffset>,
    pub location: Location,
    pub show_on_leader_board: bool,
    pub report_interval: Vec<ReportIntervals>,
    pub preferences: Preferences,
    pub banned: bool,
    #[digestible(digest_with = digest_with_hash)]
    pub last_logged_in: DateTime<FixedOffset>,
    #[digestible(digest_with = digest_with_hash)]
    pub created: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema, Digestible)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromQueryResult))]
pub struct PublicUser {
    pub id: i64,
    pub name: String,
    pub username: Username,
    pub bio: Bio,
    pub group: Group,
    pub show_on_leader_board: bool,
    pub preferences: Preferences,
    pub banned: bool,
    #[sea_orm(skip)]
    pub is_connected_to_github: bool,
    #[digestible(digest_with = digest_with_hash)]
    pub last_logged_in: DateTime<FixedOffset>,
    #[digestible(digest_with = digest_with_hash)]
    pub created: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema, Digestible)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromQueryResult))]
pub struct TinyUser {
    pub id: i64,
    pub name: String,
    pub username: Username,
    pub group: Group,
    pub show_on_leader_board: bool,
    pub preferences: Preferences,
    pub banned: bool,
}
