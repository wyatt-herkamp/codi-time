// It is my internal code. I can do what I want.
#![allow(async_fn_in_trait)]
pub mod api_keys;
pub mod avatar;
pub mod connections;
pub mod custom_languages;
pub mod gravatar;
pub mod heartbeats;
pub mod projects;
pub mod teams;
pub mod users;
pub use avatar::Source;
pub use connections::Application;
use sea_orm_exports::export_module;

export_module!(users, User, has_relation);
export_module!(avatar, Avatar, has_relation);
export_module!(connections, Connection, has_relation);
export_module!(api_keys, APIKey, has_relation);

export_module!(projects, Project, has_relation);
export_module!(heartbeats, Heartbeat, has_relation);
export_module!(teams, Team, has_relation);
export_module!(teams::team_members, TeamMember, has_relation);
export_module!(custom_languages::languages, Language, has_relation);
export_module!(custom_languages::categories, LanguageCategory, has_relation);
pub static COLLATE_IGNORE_CASE: &str = "COLLATE ignoreCase";
