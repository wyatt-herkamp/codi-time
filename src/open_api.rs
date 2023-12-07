use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
        Components, ComponentsBuilder, ContactBuilder, InfoBuilder, LicenseBuilder, OpenApiBuilder,
        Paths, PathsBuilder,
    },
    OpenApi,
};

pub const API_KEY: &str = "api_key";
pub const SESSION: &str = "session";
pub struct ApiDoc;
impl ApiDoc {
    fn components() -> Components {
        let builder = ComponentsBuilder::new()
            .security_scheme(
                API_KEY,
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
            .security_scheme(
                SESSION,
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session"))),
            );
        let builder = common::register_schemas(builder)
            .schema_from::<crate::user::update_routes::UpdateBio>()
            .schema_from::<crate::user::update_routes::UpdateCore>()
            .schema_from::<crate::user::update_routes::UpdatePassword>()
            .schema_from::<crate::user::update_routes::UpdatePasswordResponse>()
            .schema_from::<crate::user::update_routes::UpdatePreferences>()
            .schema_from::<crate::recaptcha::PublicRecaptcha>()
            .schema_from::<crate::state::State>();
        builder.build()
    }
    fn paths() -> Paths {
        PathsBuilder::new()
            .path_from::<crate::user::routes::get_user>()
            .path_from::<crate::user::routes::me>()
            .path_from::<crate::user::update_routes::update_bio>()
            .path_from::<crate::user::update_routes::update_core>()
            .path_from::<crate::user::update_routes::update_password>()
            .path_from::<crate::user::update_routes::update_report_intervals>()
            .path_from::<crate::user::update_routes::update_preferences>()
            .path_from::<crate::get_state>()
            .build()
    }
}
impl OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        OpenApiBuilder::new()
            .info(
                InfoBuilder::new()
                    .title("codi-time")
                    .version(env!("CARGO_PKG_VERSION"))
                    .description(option_env!("CARGO_PKG_DESCRIPTION"))
                    .contact(Some(
                        ContactBuilder::default()
                            .name(Some("Wyatt Herkamp"))
                            .email(Some("wherkamp@gmail.com"))
                            .url(Some("https://github.com/wyatt-herkamp/codi-time"))
                            .build(),
                    ))
                    .license(Some(LicenseBuilder::default().name("Apache-2.0").build())),
            )
            .paths(Self::paths())
            .components(Some(Self::components()))
            .build()
    }
}
