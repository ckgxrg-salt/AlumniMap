use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "people",
            &[
            ("id_num", ColType::Integer),
            ("cn_name", ColType::String),
            ("en_name", ColType::StringNull),
            ("class_of", ColType::Integer),
            ("major", ColType::StringNull),
            ("profile", ColType::StringNull),
            ("email", ColType::StringNull),
            ("qq", ColType::StringNull),
            ("wechat", ColType::StringNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "people").await
    }
}
