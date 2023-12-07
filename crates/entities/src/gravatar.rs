use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct GravatarData {
    pub entry: Vec<Entry>,
}

impl GravatarData {
    pub fn get_avatar_url(&self) -> Option<String> {
        if self.entry.len() > 0 {
            Some(self.entry[0].thumbnail_url.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub hash: String,
    pub request_hash: String,
    pub profile_url: String,
    pub preferred_username: String,
    pub thumbnail_url: String,
    pub photos: Vec<Photo>,
    #[serde(rename = "last_profile_edit")]
    pub last_profile_edit: String,
    #[serde(rename = "hidden_avatar")]
    pub hidden_avatar: bool,
    pub name: Name,
    pub display_name: String,
    pub pronouns: String,
    pub urls: Vec<Url>,
    #[serde(rename = "share_flags")]
    pub share_flags: ShareFlags,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub given_name: String,
    pub family_name: String,
    pub formatted: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Photo {
    pub value: String,
    #[serde(rename = "type")]
    pub photo_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareFlags {
    pub search_engines: bool,
    pub large_language_models: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    pub value: String,
    pub title: String,
}
