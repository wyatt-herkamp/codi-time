use derive_more::{Deref, From, Into};
use sea_orm::{
    prelude::{DateTimeWithTimeZone, *},
    FromQueryResult, JsonValue,
};
use serde::{Deserialize, Serialize};

use super::Entity as ConnectionEntity;
use crate::connections::Application;
#[derive(Default, From, Into, Deref, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserConnections(pub Vec<PubConnection>);
impl UserConnections {
    pub fn get_by_type(&self, application: Application) -> Option<&PubConnection> {
        self.0.iter().find(|x| x.application == application)
    }
    /// Ensures that only one connection per application is present
    pub fn is_valid(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for connection in &self.0 {
            if seen.contains(&connection.application) {
                return false;
            }
            seen.insert(connection.application);
        }
        true
    }
}
#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "ConnectionEntity")]
struct RestOfConnection {
    other_data_private: Option<JsonValue>,
    token: String,
    expires_at: Option<DateTimeWithTimeZone>,
}
#[derive(FromQueryResult, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PubConnection {
    pub id: i64,
    pub user_id: i64,
    pub other_data: Option<JsonValue>,
    pub application: Application,
    pub created: DateTimeWithTimeZone,
}
impl PubConnection {
    /// Converts a PubConnection into a FullConnection
    pub async fn into_full(self, db: &DatabaseConnection) -> Result<Option<super::Model>, DbErr> {
        let Self {
            id,
            user_id,
            other_data,
            application,
            created,
        } = self;
        let query = ConnectionEntity::find_by_id(id)
            .into_partial_model::<RestOfConnection>()
            .one(db)
            .await?;

        let Some(RestOfConnection {
            other_data_private,
            token,
            expires_at,
            ..
        }) = query
        else {
            return Ok(None);
        };

        Ok(Some(super::Model {
            id,
            user_id,
            other_data,
            other_data_private,
            application,
            token,
            expires_at,
            created,
        }))
    }
}

impl PubConnection {
    pub async fn get_connections(
        connection: &impl ConnectionTrait,
        user_id: i64,
    ) -> Result<Vec<Self>, DbErr> {
        super::Entity::find()
            .filter(super::Column::UserId.eq(user_id))
            .into_model::<Self>()
            .all(connection)
            .await
    }
}
