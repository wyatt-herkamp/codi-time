use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        // TODO User needs a custom create for the `ignoreCase` collation
        crate::utils::entities!(
            schema,
            manager,
            entities::UserEntity,
            entities::AvatarEntity,
            entities::ConnectionEntity,
            entities::APIKeyEntity,
            entities::TeamEntity,
            entities::TeamMemberEntity,
            entities::ProjectEntity,
            entities::HeartbeatEntity,
            entities::LanguageEntity,
            entities::LanguageCategoryEntity
        );
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Implement down migration
        todo!();
    }
}
