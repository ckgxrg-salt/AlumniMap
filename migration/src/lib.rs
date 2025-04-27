#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250427_101342_universities;
mod m20250427_102012_people;
mod m20250427_102355_university_people;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250427_101342_universities::Migration),
            Box::new(m20250427_102012_people::Migration),
            Box::new(m20250427_102355_university_people::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}