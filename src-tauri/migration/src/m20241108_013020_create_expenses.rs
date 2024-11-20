use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(pk_auto(Expenses::Id))
                    .col(ColumnDef::new(Expenses::Name).text().not_null())
                    .col(ColumnDef::new(Expenses::Price).decimal().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Expenses {
    Table,
    Id,
    Name,
    Price,
}
