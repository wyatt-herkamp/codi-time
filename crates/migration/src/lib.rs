pub use sea_orm_migration::prelude::*;

mod m20230822_185310_init;
mod m20231204_154044_create_table;
pub mod utils;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230822_185310_init::Migration),
            Box::new(m20231204_154044_create_table::Migration),
        ]
    }
}
