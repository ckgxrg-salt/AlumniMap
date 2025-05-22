use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum University {
    Table,
    Id,
    Title,
    Icon,
    Colour,
    Longitude,
    Latitude,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(University::Table)
                    .if_not_exists()
                    .col(pk_auto(University::Id))
                    .col(string(University::Title))
                    .col(string(University::Icon))
                    .col(string(University::Colour))
                    .col(float(University::Longitude))
                    .col(float(University::Latitude))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(University::Table).to_owned())
            .await
    }
}
