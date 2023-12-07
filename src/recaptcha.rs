use actix_web::HttpRequest;
use anyhow::Ok;
use digestible::Digestible;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};
use utoipa::ToSchema;
#[derive(Debug, Clone, Deserialize, Serialize, Digestible, ToSchema)]
#[serde(default)]
pub struct PublicRecaptcha {
    pub site_key: String,
    pub require_on_registration: bool,
    pub require_on_login: bool,
    pub require_on_password_reset: bool,
}
impl Default for PublicRecaptcha {
    fn default() -> Self {
        Self {
            site_key: String::default(),
            require_on_registration: true,
            require_on_login: true,
            require_on_password_reset: true,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoogleRecaptcha {
    pub secret_key: String,
    #[serde(flatten)]
    pub public_config: PublicRecaptcha,
}
impl Default for GoogleRecaptcha {
    fn default() -> Self {
        Self {
            secret_key: String::default(),
            public_config: Default::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RecaptchaAccess {
    pub settings: Option<GoogleRecaptcha>,
    pub http_client: Client,
}
fn new_client() -> anyhow::Result<Client> {
    let client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;
    Ok(client)
}
#[derive(Debug, Serialize)]
struct VerifyRequest<'a> {
    secret: &'a str,
    response: &'a str,
    #[serde(rename = "remoteip", skip_serializing_if = "Option::is_none")]
    remote_ip: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RecaptchaVerificationResult {
    pub score: f64,
    pub action: String,
    pub challenge_ts: String,
    pub hostname: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ReCaptchaVerificationResponse {
    pub success: bool,
    #[serde(rename = "error-codes", default)]
    pub error_codes: Vec<String>,
    #[serde(flatten)]
    pub result: Option<RecaptchaVerificationResult>,
}
impl RecaptchaAccess {
    pub fn new(settings: Option<GoogleRecaptcha>) -> anyhow::Result<Self> {
        if let Some(settings) = settings.as_ref() {
            if settings.secret_key.is_empty() && settings.public_config.site_key.is_empty() {
                info!("Empty recaptcha settings, disabling recaptcha");
                return Ok(Self {
                    settings: None,
                    http_client: new_client()?,
                });
            } else if settings.secret_key.is_empty() {
                warn!("Recaptcha secret key is empty, disabling recaptcha");
                return Ok(Self {
                    settings: None,
                    http_client: new_client()?,
                });
            } else if settings.public_config.site_key.is_empty() {
                warn!("Recaptcha site key is empty, disabling recaptcha");
                return Ok(Self {
                    settings: None,
                    http_client: new_client()?,
                });
            }
        }
        Ok(Self {
            settings,
            http_client: new_client()?,
        })
    }

    pub fn state_value(&self) -> Option<PublicRecaptcha> {
        self.settings.as_ref().map(|s| s.public_config.clone())
    }
    pub fn require_on_registration(&self) -> bool {
        self.settings
            .as_ref()
            .map(|s| s.public_config.require_on_registration)
            .unwrap_or_default()
    }
    pub fn require_on_login(&self) -> bool {
        self.settings
            .as_ref()
            .map(|s| s.public_config.require_on_login)
            .unwrap_or_default()
    }
    #[instrument(skip(request, self))]
    pub async fn verify_response(
        &self,
        response: &str,
        request: Option<&HttpRequest>,
    ) -> anyhow::Result<bool> {
        let Some(settings) = self.settings.as_ref() else {
            // Recaptcha is disabled
            return Ok(true);
        };
        let mut verify = VerifyRequest {
            secret: &settings.secret_key,
            response,
            remote_ip: None,
        };
        let connection_info = request.map(|r| r.connection_info());
        if let Some(connection_info) = connection_info.as_ref() {
            verify.remote_ip = connection_info.realip_remote_addr();
        }
        let response = self
            .http_client
            .post("https://www.google.com/recaptcha/api/siteverify")
            .form(&verify)
            .send()
            .await?;

        let response = response.json::<ReCaptchaVerificationResponse>().await?;
        if !response.success {
            info!("Recaptcha failed: {:?}", response);
            return Ok(false);
        }
        if let Some(result) = response.result {
            debug!("Recaptcha result: {:?}", result);
            Ok(result.score > 0.5)
        } else {
            warn!("Recaptcha result is empty");
            Ok(false)
        }
    }
}
