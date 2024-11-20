use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241108_011927_create_raves::Raves, m20241108_013020_create_expenses::Expenses};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .add_column(
                        ColumnDef::new(name("rave_id"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_expenses_raves")
                    .from(Expenses::Table, Alias::new("rave_id"))
                    .to(Raves::Table, Raves::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_expenses_raves")
                    .table(Expenses::Table)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .drop_column(Alias::new("rave_id"))
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}

// #[derive(DeriveIden)]
// enum Post {
//     Table,
//     Id,
//     Title,
//     Text,
// }
