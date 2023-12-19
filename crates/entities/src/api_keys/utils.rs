use common::{APIToken, User};
use sea_orm::entity::prelude::*;
use tracing::warn;

use crate::{APIKeyColumn, APIKeyEntity, UserEntity};
pub async fn get_user_and_token(
    token: &str,
    database: &impl ConnectionTrait,
) -> Result<Option<(APIToken, User)>, DbErr> {
    let result = APIKeyEntity::find()
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
        .select_also(UserEntity)
        .into_model::<APIToken, User>()
        .one(database)
        .await?;

    match result {
        Some((api_key, Some(user))) => {
            return Ok(Some((api_key, user)));
        }
        Some((api_key, None)) => {
            warn!("API Key {} has no user", api_key.id);
            return Ok(None);
        }
        None => {
            return Ok(None);
        }
    }
}
