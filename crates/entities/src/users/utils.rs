use common::{user_types::group::Group, Email, PublicUser, User, Username};
use sea_orm::{
    entity::prelude::*, sea_query::SimpleExpr, ActiveValue, IntoActiveModel, QuerySelect,
};

use super::UserModel;
use crate::{UserColumn, UserEntity};
pub async fn does_email_exist(
    email: Email,
    connection: &impl ConnectionTrait,
) -> Result<bool, DbErr> {
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(email))
        .count(connection)
        .await?;
    Ok(user > 0)
}

pub async fn does_username_exist(
    username: Username,
    connection: &impl ConnectionTrait,
) -> Result<bool, DbErr> {
    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(username))
        .count(connection)
        .await?;
    Ok(user > 0)
}

pub async fn does_username_or_email_exist(
    username: Username,
    email: Email,
    connection: &impl ConnectionTrait,
) -> Result<bool, DbErr> {
    let user = UserEntity::find()
        .filter(
            UserColumn::Username
                .eq(username.clone())
                .or(UserColumn::Email.eq(email)),
        )
        .count(connection)
        .await?;
    Ok(user > 0)
}

pub async fn get_password(
    id: i64,
    connection: &impl ConnectionTrait,
) -> Result<Option<String>, DbErr> {
    let user: Option<String> = UserEntity::find()
        .select_only()
        .filter(UserColumn::Id.eq(id))
        .column(UserColumn::Password)
        .into_tuple()
        .one(connection)
        .await?;
    Ok(user)
}
/// A set of utilities for getting users
pub trait UserType {
    fn id(&self) -> i64;
    async fn update_password(
        &self,
        connection: &impl ConnectionTrait,
        password: String,
    ) -> Result<bool, DbErr> {
        let user = UserEntity::find_by_id(self.id()).one(connection).await?;
        if let Some(user) = user {
            let mut user = user.into_active_model();
            user.password = ActiveValue::Set(password);
            user.require_password_change = ActiveValue::Set(false);
            user.save(connection).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn get_user(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized;

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized;

    async fn get_by_email(
        connection: &impl ConnectionTrait,
        email: String,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        Self::get_user(connection, UserColumn::Email.eq(email)).await
    }

    async fn get_by_username(
        connection: &impl ConnectionTrait,
        username: String,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        Self::get_user(connection, UserColumn::Username.eq(username)).await
    }

    async fn get_by_id(connection: &impl ConnectionTrait, id: i64) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        Self::get_user(connection, UserColumn::Id.eq(id)).await
    }

    async fn get_by_group(
        connection: &impl ConnectionTrait,
        group: Group,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        Self::get_users(connection, UserColumn::Group.eq(group)).await
    }
    async fn get_user_by_username_or_email(
        username: String,
        database: &impl ConnectionTrait,
    ) -> Result<Option<Self>, DbErr>
    where
        Self: Sized,
    {
        if username.contains('@') {
            Self::get_user(database, UserColumn::Email.eq(username)).await
        } else {
            Self::get_user(database, UserColumn::Email.eq(username)).await
        }
    }
}
impl UserType for UserModel {
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
        UserEntity::find().filter(filter).one(connection).await
    }

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        UserEntity::find().filter(filter).all(connection).await
    }
}
impl UserType for PublicUser {
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
        let user = UserEntity::find()
            .filter(filter)
            .into_model()
            .one(connection)
            .await?;
        Ok(user)
    }

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        let users = UserEntity::find()
            .filter(filter)
            .into_model()
            .all(connection)
            .await?;
        Ok(users)
    }
}
impl UserType for User {
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
        let user = UserEntity::find()
            .filter(filter)
            .into_model()
            .one(connection)
            .await?;
        Ok(user)
    }

    async fn get_users(
        connection: &impl ConnectionTrait,
        filter: SimpleExpr,
    ) -> Result<Vec<Self>, DbErr>
    where
        Self: Sized,
    {
        let users = UserEntity::find()
            .filter(filter)
            .into_model()
            .all(connection)
            .await?;
        Ok(users)
    }
}
impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            group: value.group,
            banned: value.banned,
            created: value.created,
            name: value.name,
            bio: value.bio,
            email_verified_at: value.email_verified_at,
            receive_email_notifications: value.receive_email_notifications,
            require_password_change: value.require_password_change,
            password_changed_at: value.password_changed_at,
            location: value.location,
            show_on_leader_board: value.show_on_leader_board,
            report_interval: value.report_interval,
            preferences: value.preferences,
            last_logged_in: value.last_logged_in,
        }
    }
}
pub async fn does_first_user_exist(connection: &impl ConnectionTrait) -> Result<bool, DbErr> {
    let num_of_users = UserEntity::find().count(connection).await?;
    Ok(num_of_users > 0)
}
