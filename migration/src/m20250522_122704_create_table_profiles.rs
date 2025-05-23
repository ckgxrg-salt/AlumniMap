use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Profile {
    Table,
    Id,
    NamePrimary,
    NameSupplementary,
    Avatar,
    ClassOf,
    UniversityId,
    Major,
    Bio,
    Email,
    QQ,
    Wechat,
    Matrix,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(pk_auto(Profile::Id))
                    .col(string(Profile::NamePrimary))
                    .col(string_null(Profile::NameSupplementary))
                    .col(string(Profile::Avatar))
                    .col(integer(Profile::ClassOf))
                    .col(string_null(Profile::Major))
                    .col(integer(Profile::UniversityId))
                    .col(string_null(Profile::Bio))
                    .col(string_null(Profile::Email))
                    .col(string_null(Profile::QQ))
                    .col(string_null(Profile::Wechat))
                    .col(string_null(Profile::Matrix))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}
