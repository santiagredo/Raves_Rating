use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241108_010513_create_artists::Artists, m20241108_011927_create_raves::Raves};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RaveArtist::Table)
                    .if_not_exists()
                    .col(pk_auto(RaveArtist::Id))
                    .col(ColumnDef::new(RaveArtist::RaveId).integer().not_null())
                    .col(ColumnDef::new(RaveArtist::ArtistId).integer().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_raveartist_rave")
                    .from(RaveArtist::Table, RaveArtist::RaveId)
                    .to(Raves::Table, Raves::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_raveartist_artist")
                    .from(RaveArtist::Table, RaveArtist::ArtistId)
                    .to(Artists::Table, Artists::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .name("unique_rave_artist")
                    .table(RaveArtist::Table)
                    .col(RaveArtist::RaveId)
                    .col(RaveArtist::ArtistId)
                    .unique()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RaveArtist::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RaveArtist {
    Table,
    Id,
    RaveId,
    ArtistId,
}
