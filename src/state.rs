use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{DateTime, Utc};
use digestible::Digestible;
use serde::Serialize;
use utoipa::ToSchema;

use crate::recaptcha::PublicRecaptcha;
#[derive(Debug, Digestible, Serialize, ToSchema)]
pub struct State {
    /// True if the first user has not been created yet
    /// False if the first user has been created
    #[digestible(digest_with = atomics::digest_relaxed)]
    #[schema(value_type = bool)]
    pub is_first_user: AtomicBool,
    pub public_registration: bool,
    /// URL to the home page
    pub home_url: Option<String>,
    pub recaptcha_config: Option<PublicRecaptcha>,
    #[digestible(digest_with = digest_with_hash)]
    pub started_at: DateTime<Utc>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_first_user: Default::default(),
            public_registration: true,
            started_at: Utc::now(),
            recaptcha_config: None,
            home_url: None,
        }
    }
}
impl State {
    pub fn is_first_user(&self) -> bool {
        self.is_first_user.load(Ordering::Relaxed)
    }
    pub fn created_first_user(&self) {
        self.is_first_user.store(false, Ordering::Relaxed);
    }
}
