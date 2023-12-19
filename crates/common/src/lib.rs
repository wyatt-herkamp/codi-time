pub mod user_types;
pub use http;
pub use project::{PartialProjectQuery, Project, ProjectQuery, ProjectSortBy, UserOrTeam};
pub use query_params::*;
pub use user_types::{
    api_token::{APIToken, APITokenPermissions},
    bio::{Bio, Pronouns},
    group::Group,
    preferences::Preferences,
    public_user::{PublicUser, TinyUser, User},
    report_intervals::ReportIntervals,
    Email, Username,
};
#[cfg(feature = "sea-orm")]
pub mod database_helpers;
pub mod project;
pub mod query_params;
use utoipa::openapi::ComponentsBuilder;
pub use version_control_ref::*;
pub mod heartbeat;
pub mod language;
pub mod locations;
pub mod version_control_ref;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
use utoipa::ToSchema;
pub fn register_schemas(builder: ComponentsBuilder) -> ComponentsBuilder {
    builder
        .schema_from::<User>()
        .schema_from::<Bio>()
        .schema_from::<Pronouns>()
        .schema_from::<APITokenPermissions>()
        .schema_from::<Preferences>()
        .schema_from::<Username>()
        .schema_from::<Email>()
        .schema_from::<ReportIntervals>()
        .schema_from::<Group>()
        .schema_from::<TinyUser>()
        .schema_from::<PublicUser>()
        .schema_from::<Project>()
        .schema_from::<VersionControlRef>()
        .schema_from::<GithubVersionControlRef>()
        .schema_from::<PRStatus>()
        .schema_from::<PullRequest>()
        .schema_from::<IdOrName>()
        .schema_from::<ProjectQuery>()
        .schema_from::<PartialProjectQuery>()
        .schema_from::<UserOrTeam>()
        .schema_from::<ProjectSortBy>()
        .schema_from::<QueryOrdering>()
}

/// Accepts either an integer id or a string name.
#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIs, ToSchema, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(i64),
    Name(String),
}
impl From<i64> for IdOrName {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<String> for IdOrName {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

impl From<&str> for IdOrName {
    fn from(name: &str) -> Self {
        Self::Name(name.to_string())
    }
}
