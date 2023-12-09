use ahash::{HashMap, HashMapExt};
use chrono::{DateTime, Local};
use common::{user_types::api_token::FromCLI, APIToken};
use parking_lot::RwLock;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewCLIRequest {
    pub from_cli: FromCLI,
    pub username: Option<String>,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletedRequest {
    pub from_cli: FromCLI,
    pub username: Option<String>,
    pub api_token: APIToken,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub completed_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct CLIAccess {
    pub pending_accesses: RwLock<HashMap<String, NewCLIRequest>>,
    pub unclaimed_accesses: RwLock<HashMap<String, CompletedRequest>>,
}

impl CLIAccess {
    pub fn new() -> Self {
        Self {
            pending_accesses: RwLock::new(HashMap::new()),
            unclaimed_accesses: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_new_pending_access(
        &self,
        from_cli: FromCLI,
        username: impl Into<Option<String>>,
        ip_address: impl Into<String>,
    ) -> String {
        let mut pending_accesses = self.pending_accesses.write();
        let key: String = loop {
            let rand = StdRng::from_entropy();
            let key = rand
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            if !pending_accesses.contains_key(&key) {
                break key;
            }
        };
        let new_access = NewCLIRequest {
            from_cli,
            username: username.into(),
            ip_address: ip_address.into(),
            created_at: Local::now(),
        };
        pending_accesses.insert(key.clone(), new_access);
        key
    }

    pub fn get_pending_access(&self, key: &str) -> Option<NewCLIRequest> {
        let pending_accesses = self.pending_accesses.read();
        pending_accesses.get(key).cloned()
    }
    pub fn remove_pending_access(&self, key: &str) -> Option<NewCLIRequest> {
        let mut pending_accesses = self.pending_accesses.write();
        pending_accesses.remove(key)
    }

    pub fn get_unclaimed_access(&self, key: &str) -> Option<CompletedRequest> {
        let mut unclaimed_accesses = self.unclaimed_accesses.write();
        unclaimed_accesses.remove(key)
    }
}
