use std::{convert::Infallible, fmt::Debug, sync::Arc};

use ahash::HashMapExt;
use chrono::Duration;
use parking_lot::RwLock;
use tracing::instrument;

use crate::{
    config::SessionConfig,
    user::session::{Session, SessionManager},
};

pub type SessionMap = ahash::HashMap<String, Session>;
pub struct MemorySessionManager {
    sessions: RwLock<SessionMap>,
    config: Arc<SessionConfig>,
}
impl Debug for MemorySessionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemorySessionManager")
            .field("sessions", &self.sessions.read().len())
            .field("config", &self.config)
            .finish()
    }
}
impl SessionManager for MemorySessionManager {
    type Error = Infallible;
    type Config = usize;

    fn new(start_size: Self::Config, session_config: SessionConfig) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            sessions: RwLock::new(SessionMap::with_capacity(start_size)),
            config: Arc::new(session_config),
        })
    }
    #[instrument]
    fn create_session_with_life(
        &self,
        user_id: i64,
        life: Duration,
    ) -> Result<Session, Infallible> {
        let mut sessions = self.sessions.write();

        let session_id = super::create_session_id(|x| sessions.contains_key(x));
        let session = Session::new(user_id, session_id.clone(), life);
        sessions.insert(session_id, session.clone());
        Ok(session)
    }
    #[instrument]
    fn get_session(&self, session_id: &str) -> Result<Option<Session>, Infallible> {
        let sessions = self.sessions.read();
        Ok(sessions.get(session_id).cloned())
    }
    #[instrument]
    fn delete_session(&self, session_id: &str) -> Result<Option<Session>, Infallible> {
        let mut sessions = self.sessions.write();
        Ok(sessions.remove(session_id))
    }

    fn get_session_config(&self) -> Arc<SessionConfig> {
        self.config.clone()
    }

    fn get_session_config_ref(&self) -> &SessionConfig {
        &self.config
    }
}
