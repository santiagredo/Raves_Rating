use crate::m20241108_011153_create_locations::Locations;
use crate::m20241108_011410_create_rating::Ratings;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Raves::Table)
                    .if_not_exists()
                    .col(pk_auto(Raves::Id))
                    .col(ColumnDef::new(Raves::Name).text().not_null())
                    .col(ColumnDef::new(Raves::Location).integer().not_null())
                    .col(ColumnDef::new(Raves::Rating).integer().not_null())
                    .col(ColumnDef::new(Raves::Date).date().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_rave_location")
                    .from(Raves::Table, Raves::Location)
                    .to(Locations::Table, Locations::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_rave_rating")
                    .from(Raves::Table, Raves::Rating)
                    .to(Ratings::Table, Ratings::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Raves::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Raves {
    Table,
    Id,
    Name,
    Location,
    Rating,
    Date,
}
