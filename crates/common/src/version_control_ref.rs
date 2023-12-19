use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::FromJsonQueryResult))]
#[serde(tag = "type", content = "value")]
pub enum VersionControlRef {
    Github(GithubVersionControlRef),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "value")]
pub enum GithubVersionControlRef {
    Normal {
        owner: String,
        repo: String,
    },
    Fork {
        owner: String,
        repo: String,
        fork_owner: String,
        fork_repo: String,
        pull_requests: Vec<PullRequest>,
    },
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    EnumString,
    Serialize,
    Deserialize,
    ToSchema,
)]
pub enum PRStatus {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema)]
pub struct PullRequest {
    pub id: u64,
    pub status: PRStatus,
    pub from_branch: String,
    pub to_branch: String,
}
