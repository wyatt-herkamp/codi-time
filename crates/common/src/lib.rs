pub mod user_types;
pub use http;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
pub use user_types::{
    bio::{Bio, Pronouns},
    group::Group,
    key_permissions::APIKeyPermissions,
    preferences::Preferences,
    public_user::{PublicUser, TinyUser, User},
    report_intervals::ReportIntervals,
    Email, Username,
};
use utoipa::{openapi::ComponentsBuilder, PartialSchema, ToSchema};
pub mod heartbeat;
pub mod language;
pub mod locations;
pub mod version_control_ref;

pub fn register_schemas(builder: ComponentsBuilder) -> ComponentsBuilder {
    builder
        .schema_from::<User>()
        .schema_from::<Bio>()
        .schema_from::<Pronouns>()
        .schema_from::<APIKeyPermissions>()
        .schema_from::<Preferences>()
        .schema_from::<Username>()
        .schema_from::<Email>()
        .schema_from::<ReportIntervals>()
        .schema_from::<Group>()
        .schema_from::<TinyUser>()
        .schema_from::<PublicUser>()
}
