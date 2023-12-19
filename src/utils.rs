use serde::Serialize;

pub mod base64_utils {
    use base64::{engine::general_purpose::STANDARD, DecodeError, Engine};
    #[inline(always)]

    pub fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(input)
    }
    #[inline(always)]
    pub fn decode_as_string(input: impl AsRef<[u8]>) -> Result<String, actix_web::Error> {
        let decoded = decode(input).map_err(actix_web::error::ErrorBadRequest)?;
        String::from_utf8(decoded).map_err(actix_web::error::ErrorBadRequest)
    }
    #[inline(always)]
    pub fn encode(input: impl AsRef<[u8]>) -> String {
        STANDARD.encode(input)
    }
    #[inline(always)]
    pub fn encode_basic_header(username: impl AsRef<str>, password: impl AsRef<str>) -> String {
        STANDARD.encode(format!("{}:{}", username.as_ref(), password.as_ref()))
    }
}
pub mod sha256 {
    use digestible::{Digester, Digestible, IntoBase64};
    use sha2::Digest;

    use crate::utils::base64_utils;

    #[inline(always)]
    pub fn encode_to_string(input: impl AsRef<[u8]>) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        base64_utils::encode(result)
    }
    pub fn digest_to_string(digest: &impl Digestible) -> String {
        sha2::Sha256::new().into_base64().digest_big_endian(digest)
    }
}
pub mod password {

    use argon2::{
        password_hash::{Error, SaltString},
        Argon2, PasswordHasher, PasswordVerifier,
    };
    use rand::rngs::OsRng;
    use tracing::log;

    use crate::error::WebsiteError;

    pub fn encrypt_password(password: &str) -> Result<String, WebsiteError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_ref(), &salt)
            .map(|v| v.to_string())
            .map_err(WebsiteError::EncryptPasswordError)
    }
    #[inline(always)]
    pub fn check_password(password: &str, hash: &str) -> Result<bool, WebsiteError> {
        let argon2 = Argon2::default();
        let hash = match argon2::PasswordHash::new(hash) {
            Ok(ok) => ok,
            Err(err) => {
                log::error!("Error parsing password hash: {}", err);
                return Ok(false);
            }
        };
        if let Err(error) = argon2.verify_password(password.as_ref(), &hash) {
            match error {
                Error::Password => {}
                error => {
                    log::error!("Error verifying password: {}", error);
                }
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

pub mod time_utils {
    pub fn get_current_time() -> chrono::DateTime<chrono::FixedOffset> {
        chrono::Utc::now().into()
    }
}
pub mod token {
    use rand::{distributions::Distribution, rngs::StdRng, SeedableRng};

    /// Generate a random token and its hash
    pub fn generate_token() -> (String, String) {
        let mut rng = StdRng::from_entropy();
        let token: String = rand::distributions::Alphanumeric
            .sample_iter(&mut rng)
            .take(32)
            .map(char::from)
            .collect();
        let token_hash = super::sha256::encode_to_string(&token);
        (token, token_hash)
    }
}
#[derive(Serialize)]
pub struct CreateResponse<T: Serialize> {
    pub data: T,
}
