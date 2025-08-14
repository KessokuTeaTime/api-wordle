use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // `puzzles`
        manager
            .create_table(
                Table::create()
                    .table(Puzzles::Table)
                    .if_not_exists()
                    .col(date(Puzzles::Date).primary_key())
                    .col(string(Puzzles::Solution))
                    .col(boolean(Puzzles::IsDeleted))
                    .to_owned(),
            )
            .await?;

        // `sessions`
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(string(Sessions::Session).primary_key())
                    .col(date_time(Sessions::CreationDate))
                    .col(date_time(Sessions::UpdateDate))
                    .to_owned(),
            )
            .await?;

        // `histories`
        manager
            .create_table(
                Table::create()
                    .table(Histories::Table)
                    .if_not_exists()
                    .col(date(Histories::Date))
                    .col(string(Histories::Session))
                    .col(string(Histories::History))
                    .col(date_time(Histories::UploadDate))
                    .primary_key(Index::create().col(Histories::Date).col(Histories::Session))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_puzzle_date")
                            .from(Histories::Table, Histories::Date)
                            .to(Puzzles::Table, Puzzles::Date)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session")
                            .from(Histories::Table, Histories::Session)
                            .to(Sessions::Table, Sessions::Session)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // `histories`
        manager
            .drop_table(Table::drop().table(Histories::Table).to_owned())
            .await?;

        // `sessions`
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await?;

        // `puzzles`
        manager
            .drop_table(Table::drop().table(Puzzles::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Puzzles {
    Table,
    Date,
    Solution,
    IsDeleted,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Session,
    #[sea_orm(iden = "created_at")]
    CreationDate,
    #[sea_orm(iden = "updated_at")]
    UpdateDate,
}

#[derive(DeriveIden)]
enum Histories {
    Table,
    Date,
    Session,
    History,
    #[sea_orm(iden = "uploaded_at")]
    UploadDate,
}
