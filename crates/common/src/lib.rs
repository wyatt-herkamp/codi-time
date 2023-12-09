pub mod user_types;
pub use http;
pub use user_types::{
    api_token::{APIToken, APITokenPermissions},
    bio::{Bio, Pronouns},
    group::Group,
    preferences::Preferences,
    public_user::{PublicUser, TinyUser, User},
    report_intervals::ReportIntervals,
    Email, Username,
};
use utoipa::openapi::ComponentsBuilder;
pub mod heartbeat;
pub mod language;
pub mod locations;
pub mod version_control_ref;

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
}
