use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ratings::Table)
                    .if_not_exists()
                    .col(pk_auto(Ratings::Id))
                    .col(ColumnDef::new(Ratings::Name).text().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_ratings = Query::insert()
            .into_table(Ratings::Table)
            .columns([Ratings::Name])
            .values_panic(["Horrible".into()])
            .values_panic(["Bad".into()])
            .values_panic(["Average".into()])
            .values_panic(["Great".into()])
            .values_panic(["Perfect".into()])
            .to_owned();

        manager.exec_stmt(insert_ratings).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ratings::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Ratings {
    Table,
    Id,
    Name,
}
