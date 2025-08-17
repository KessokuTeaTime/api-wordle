use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Histories::Table)
                    .add_column_if_not_exists(boolean(Histories::IsCompleted).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Histories::Table)
                    .drop_column(Histories::IsCompleted)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Histories {
    Table,
    IsCompleted,
}
