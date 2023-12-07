use std::{path::PathBuf, sync::Arc};

use chrono::Duration;
use redb::{Database, ReadableTable, TableDefinition};
use tracing::instrument;

use crate::{
    config::SessionConfig,
    user::session::{Session, SessionManager, SessionTuple},
};

const TABLE: TableDefinition<&str, SessionTuple> = TableDefinition::new("sessions");
#[derive(Debug)]
pub struct RedbSessionManager {
    sessions: Database,
    config: Arc<SessionConfig>,
}

impl SessionManager for RedbSessionManager {
    type Error = anyhow::Error;
    type Config = PathBuf;

    fn new(manager_config: Self::Config, session_config: SessionConfig) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let sessions = Database::open(manager_config)?;
        Ok(Self {
            sessions,
            config: Arc::new(session_config),
        })
    }
    #[instrument]
    fn create_session_with_life(
        &self,
        user_id: i64,
        life: Duration,
    ) -> Result<Session, Self::Error> {
        let sessions = self.sessions.begin_write()?;
        let mut session_table = sessions.open_table(TABLE)?;

        let session_id = super::create_session_id(|x| {
            session_table.get(x).map(|x| x.is_some()).unwrap_or(false)
        });
        let session = Session::new(user_id, session_id.clone(), life);
        session_table.insert(&*session_id, session.as_tuple_ref())?;
        drop(session_table);
        sessions.commit()?;
        Ok(session)
    }

    #[instrument]
    fn get_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error> {
        let sessions = self.sessions.begin_read()?;

        let session = sessions.open_table(TABLE)?;
        let session = session
            .get(session_id)?
            .map(|x| Session::from_tuple(x.value()));
        Ok(session)
    }
    #[instrument]
    fn delete_session(&self, session_id: &str) -> Result<Option<Session>, Self::Error> {
        let sessions = self.sessions.begin_write()?;
        let mut table = sessions.open_table(TABLE)?;
        let session = table
            .remove(session_id)?
            .map(|x| Session::from_tuple(x.value()));
        drop(table);
        sessions.commit()?;
        Ok(session)
    }

    fn get_session_config(&self) -> Arc<SessionConfig> {
        self.config.clone()
    }

    fn get_session_config_ref(&self) -> &SessionConfig {
        &self.config
    }
}
