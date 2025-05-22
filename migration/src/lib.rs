pub use sea_orm_migration::prelude::*;

mod m20250522_121745_create_table_universities;
mod m20250522_122704_create_table_profiles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250522_121745_create_table_universities::Migration),
            Box::new(m20250522_122704_create_table_profiles::Migration),
        ]
    }
}
