use common::{APIToken, User};
use sea_orm::{
    entity::prelude::*, sea_query::SimpleExpr, ActiveValue, IntoActiveModel, QuerySelect,
};
use tracing::warn;

use crate::{APIKeyColumn, APIKeyEntity, UserColumn, UserEntity};
pub async fn get_user_and_token(
    token: &str,
    database: &impl ConnectionTrait,
) -> Result<Option<(APIToken, User)>, DbErr> {
    let Some(api_key) = APIKeyEntity::find()
        .filter(
            APIKeyColumn::Token
                .eq(token)
                .and(APIKeyColumn::Revoked.is_null())
                .and(
                    APIKeyColumn::ExpiresAt
                        .is_null()
                        .or(APIKeyColumn::ExpiresAt.gte(chrono::Utc::now())),
                ),
        )
        .into_model::<APIToken>()
        .one(database)
        .await?
    else {
        return Ok(None);
    };

    let Some(user) = UserEntity::find()
        .filter(
            UserColumn::Id
                .eq(api_key.user_id)
                .and(UserColumn::Banned.eq(false)),
        )
        .into_model::<User>()
        .one(database)
        .await?
    else {
        warn!("API Key {} has no user", api_key.id);
        return Ok(None);
    };
    Ok(Some((api_key, user)))
}
