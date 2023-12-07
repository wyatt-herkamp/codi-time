mod dyn_session;
pub mod memory;
pub mod redb_session;

use std::{fmt::Debug, sync::Arc};

use actix_web::web::Data;
use chrono::{DateTime, Duration, Utc};
use digestible::Digestible;
pub use dyn_session::{DynSessionManager, SessionError};
use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use tracing::instrument;

use crate::config::SessionConfig;

pub type SessionManagerType = DynSessionManager;

/// A session type.
/// Stored in the session manager.
///
/// Redb converts this to a tuple of (user_id, session_id, expires, created)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Digestible)]
pub struct Session {
    pub user_id: i64,
    pub session_id: String,
    #[digestible(digest_with = digest_with_hash)]
    pub expires: DateTime<Utc>,
    #[digestible(digest_with = digest_with_hash)]
    pub created: DateTime<Utc>,
}
impl Session {
    pub fn new(user_id: i64, session_id: String, life: Duration) -> Session {
        Session {
            user_id,
            session_id,
            expires: Utc::now() + life,
            created: Utc::now(),
        }
    }
    pub fn is_expired(&self) -> bool {
        self.expires < Utc::now()
    }
    pub fn from_tuple(tuple: SessionTuple) -> Self {
        let (user_id, session_id, expires, created) = tuple;
        Session {
            user_id,
            session_id: session_id.to_owned(),
            expires: DateTime::from_naive_utc_and_offset(
                chrono::NaiveDateTime::from_timestamp_millis(expires).unwrap_or_default(),
                Utc,
            ),
            created: DateTime::from_naive_utc_and_offset(
                chrono::NaiveDateTime::from_timestamp_millis(created).unwrap_or_default(),
                Utc,
            ),
        }
    }
    pub fn as_tuple_ref(&self) -> SessionTuple {
        (
            self.user_id,
            self.session_id.as_str(),
            self.expires.timestamp_millis(),
            self.created.timestamp_millis(),
        )
    }
}
/// A tuple of (user_id, session_id, expires, created)
pub type SessionTuple<'value> = (i64, &'value str, i64, i64);

pub trait SessionManager: Debug {
    type Error;
    type Config;

    fn new(
        manager_config: Self::Config,
        session_config: SessionConfig,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn create_session_with_life(
        &self,
        user_id: i64,
        life: Duration,
    ) -> Result<Session, Self::Error>;
    #[instrument]
    fn create_session(&self, user_id: i64) -> Result<Session, Self::Error> {
        self.create_session_with_life(
            user_id,
            self.get_session_config_ref()
                .session_lifetime
                .duration
                .clone(),
        )
    }
    fn get_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error>;

    fn delete_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error>;
    fn get_session_config(&self) -> Arc<SessionConfig>;
    fn get_session_config_ref(&self) -> &SessionConfig;
}
impl<T: SessionManager> SessionManager for Data<T> {
    type Error = T::Error;
    type Config = T::Config;

    fn new(manager_config: Self::Config, session_config: SessionConfig) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        T::new(manager_config, session_config).map(Data::new)
    }

    fn create_session_with_life(
        &self,
        user_id: i64,
        life: Duration,
    ) -> Result<Session, Self::Error> {
        self.get_ref().create_session_with_life(user_id, life)
    }

    fn get_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error> {
        self.get_ref().get_session(session_id)
    }

    fn delete_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error> {
        self.get_ref().delete_session(session_id)
    }

    fn get_session_config(&self) -> Arc<SessionConfig> {
        self.get_ref().get_session_config()
    }

    fn get_session_config_ref(&self) -> &SessionConfig {
        self.get_ref().get_session_config_ref()
    }
}

#[inline(always)]
pub fn create_session_id(exists_call_back: impl Fn(&str) -> bool) -> String {
    let mut rand = StdRng::from_entropy();
    loop {
        let session_id: String = (0..7).map(|_| rand.sample(Alphanumeric) as char).collect();
        if !exists_call_back(&session_id) {
            break session_id;
        }
    }
}
