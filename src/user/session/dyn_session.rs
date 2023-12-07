use std::{convert::Infallible, sync::Arc};

use chrono::Duration;
use thiserror::Error;

use crate::{
    config::{SessionConfig, SessionManagerConfig},
    user::session::{
        memory::MemorySessionManager, redb_session::RedbSessionManager, Session, SessionManager,
    },
};
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SessionError {
    #[error("Infallible. This is cool.")]
    Infallible,
    #[error(
        r#"Redb Error: Must likely the database is locked or corrupted. 
    These sessions are short lived. 
    So just delete the database and restart the server. If this problem persists. Please report it. 
    {0}
    "#
    )]
    RedbError(anyhow::Error),
}
impl From<()> for SessionError {
    fn from(_: ()) -> Self {
        SessionError::Infallible
    }
}
impl From<Infallible> for SessionError {
    fn from(_: Infallible) -> Self {
        SessionError::Infallible
    }
}
#[derive(Debug)]
#[non_exhaustive]
pub enum DynSessionManager {
    Memory(MemorySessionManager),
    Redb(RedbSessionManager),
}

impl SessionManager for DynSessionManager {
    type Error = SessionError;
    type Config = SessionManagerConfig;

    fn new(manager_config: Self::Config, session_config: SessionConfig) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match manager_config {
            SessionManagerConfig::Memory { start_size } => Ok(DynSessionManager::Memory(
                MemorySessionManager::new(start_size, session_config)?,
            )),
            SessionManagerConfig::Redb { file } => Ok(DynSessionManager::Redb(
                RedbSessionManager::new(file, session_config)
                    .map_err(|v| SessionError::RedbError(v))?,
            )),
        }
    }

    fn create_session_with_life(
        &self,
        user_id: i64,
        life: Duration,
    ) -> Result<Session, SessionError> {
        match self {
            DynSessionManager::Memory(session) => session
                .create_session_with_life(user_id, life)
                .map_err(|_| SessionError::Infallible),
            DynSessionManager::Redb(session) => session
                .create_session_with_life(user_id, life)
                .map_err(|x| SessionError::RedbError(x)),
        }
    }

    fn get_session(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
        match self {
            DynSessionManager::Memory(session) => session
                .get_session(session_id)
                .map_err(|_| SessionError::Infallible),
            DynSessionManager::Redb(session) => session
                .get_session(session_id)
                .map_err(|x| SessionError::RedbError(x)),
        }
    }

    fn delete_session(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
        match self {
            DynSessionManager::Memory(session) => session
                .delete_session(session_id)
                .map_err(|_| SessionError::Infallible),
            DynSessionManager::Redb(session) => session
                .delete_session(session_id)
                .map_err(|x| SessionError::RedbError(x)),
        }
    }

    fn get_session_config(&self) -> Arc<SessionConfig> {
        match self {
            DynSessionManager::Memory(session) => session.get_session_config(),
            DynSessionManager::Redb(session) => session.get_session_config(),
        }
    }

    fn get_session_config_ref(&self) -> &SessionConfig {
        match self {
            DynSessionManager::Memory(session) => session.get_session_config_ref(),
            DynSessionManager::Redb(session) => session.get_session_config_ref(),
        }
    }
}
