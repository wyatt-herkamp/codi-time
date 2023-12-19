use chrono::{DateTime, FixedOffset};
use either::Either;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{version_control_ref::VersionControlRef, IdOrName, QueryOrdering};

#[derive(Clone, Debug, PartialEq, ToSchema, Serialize, Deserialize)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromQueryResult))]
pub struct Project {
    pub id: i64,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub name: String,
    /// Other Names for the Project
    pub renames: Vec<String>,
    pub languages: Vec<String>,
    pub color: Option<String>,
    pub version_control_ref: Option<VersionControlRef>,
    pub public: bool,
    pub last_heartbeat: DateTime<FixedOffset>,
    pub last_update: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
}
#[derive(Clone, Debug, PartialEq, ToSchema, Serialize, Deserialize)]
pub enum UserOrTeam {
    User {
        id_or_name: IdOrName,
        check_teams: bool,
    },
    Team(IdOrName),
}
#[derive(Clone, Debug, PartialEq, ToSchema, Serialize, Deserialize)]
pub enum ProjectSortBy {
    Name(Option<QueryOrdering>),
    LastUpdate(Option<QueryOrdering>),
    LastHeartbeat(Option<QueryOrdering>),
}
/// A Project Query for your own Projects
#[derive(Clone, Debug, PartialEq, ToSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct PartialProjectQuery {
    pub language: Option<String>,
    /// Will Query Name and Renames
    pub name: Option<String>,
    pub sort_by: Option<ProjectSortBy>,
}

impl Default for PartialProjectQuery {
    fn default() -> Self {
        Self {
            language: None,
            name: None,
            sort_by: None,
        }
    }
}
/// A Query for Projects
#[derive(Clone, Debug, PartialEq, ToSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectQuery {
    pub owned_by: UserOrTeam,
    #[serde(flatten)]
    pub query_params: PartialProjectQuery,
    // TODO add pagination
}

impl From<(i64, PartialProjectQuery)> for ProjectQuery {
    fn from((id, query_params): (i64, PartialProjectQuery)) -> Self {
        Self {
            owned_by: UserOrTeam::User {
                id_or_name: IdOrName::Id(id),
                check_teams: true,
            },
            query_params,
        }
    }
}
impl Default for ProjectQuery {
    fn default() -> Self {
        Self {
            query_params: Default::default(),
            owned_by: UserOrTeam::User {
                id_or_name: IdOrName::Id(0),
                check_teams: true,
            },
        }
    }
}

impl From<i64> for ProjectQuery {
    fn from(id: i64) -> Self {
        Self {
            owned_by: UserOrTeam::User {
                id_or_name: id.into(),
                check_teams: true,
            },
            ..Default::default()
        }
    }
}
