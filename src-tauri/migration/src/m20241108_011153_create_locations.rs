use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Locations::Table)
                    .if_not_exists()
                    .col(pk_auto(Locations::Id))
                    .col(ColumnDef::new(Locations::Name).text().unique_key().not_null())
                    .col(ColumnDef::new(Locations::Address).text().not_null())
                    .col(ColumnDef::new(Locations::Capacity).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Locations::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Locations {
    Table,
    Id,
    Name,
    Address,
    Capacity,
}
