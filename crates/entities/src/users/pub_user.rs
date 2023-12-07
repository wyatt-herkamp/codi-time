use common::user_types::group::Group;
use sea_orm::{
    prelude::{DateTimeWithTimeZone, *},
    sea_query::SimpleExpr,
    DbErr, FromQueryResult,
};
use serde::{Deserialize, Serialize};

use super::UserType;
use crate::{
    connections::pub_connection::{PubConnection, UserConnections},
    UserEntity,
};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromQueryResult)]
pub struct TinyUser {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    // Group Details
    pub group: Group,
    pub banned: bool,
    pub created: DateTimeWithTimeZone,
}
impl UserType for TinyUser {
    fn id(&self) -> i64 {
        self.id
    }
    async fn get_user(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        UserEntity::find()
            .filter(filter)
            .into_model::<Self>()
            .one(connection)
            .await
    }

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        UserEntity::find()
            .filter(filter)
            .into_model::<Self>()
            .all(connection)
            .await
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicUser {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar: String,
    pub group: Group,
    pub banned: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub connections: UserConnections,
    pub created: DateTimeWithTimeZone,
}

impl sea_orm::FromQueryResult for PublicUser {
    fn from_query_result(row: &QueryResult, pre: &str) -> Result<Self, DbErr> {
        Ok(Self {
            id: row.try_get(pre, "id")?,
            name: row.try_get(pre, "name")?,
            username: row.try_get(pre, "username")?,
            avatar: row.try_get(pre, "avatar")?,
            group: row.try_get(pre, "group")?,
            banned: row.try_get(pre, "banned")?,
            connections: Default::default(),
            created: row.try_get(pre, "created")?,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, FromQueryResult)]
pub struct FullUser {
    pub id: i64,
    pub name: String,
    // TODO Add Avatar
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub group: Group,
    pub require_password_change: bool,
    #[sea_orm(skip)]
    pub connections: UserConnections,
    pub banned: bool,
    pub created: DateTimeWithTimeZone,
}
impl UserType for FullUser {
    async fn get_user(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        let user = UserEntity::find()
            // .column_as(AvatarColumn::Source, "avatar_source")
            // .column_as(AvatarColumn::Created, "avatar_created")
            // .join(JoinType::InnerJoin, UserRelation::Avatar.def())
            .filter(filter)
            .into_model::<Self>()
            .one(connection)
            .await?;
        if let Some(mut user) = user {
            user.connections = PubConnection::get_connections(connection, user.id)
                .await?
                .into();
            Ok(Some(user))
        } else {
            return Ok(None);
        }
    }

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        let mut user = UserEntity::find()
            //.column_as(AvatarColumn::Source, "avatar_source")
            //.column_as(AvatarColumn::Created, "avatar_created")
            //.join(JoinType::InnerJoin, UserRelation::Avatar.def())
            .filter(filter)
            .into_model::<Self>()
            .all(connection)
            .await?;
        for user in user.iter_mut() {
            user.connections = PubConnection::get_connections(connection, user.id)
                .await?
                .into();
        }
        Ok(user)
    }

    fn id(&self) -> i64 {
        self.id
    }
}
impl FullUser {}
