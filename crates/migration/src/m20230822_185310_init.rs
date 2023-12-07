use sea_orm_migration::prelude::*;
static INIT: &str = include_str!("postgres_init.sql");
static DOWN: &str = include_str!("postgres_init_down.sql");
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(INIT).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(DOWN).await?;
        Ok(())
    }
}
