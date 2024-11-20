pub use sea_orm_migration::prelude::*;

mod m20241108_010513_create_artists;
mod m20241108_011153_create_locations;
mod m20241108_011410_create_rating;
mod m20241108_011927_create_raves;
mod m20241108_013020_create_expenses;
mod m20241109_224247_create_rave_artists;
mod m20241109_224816_create_expenses_rave_fk;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241108_010513_create_artists::Migration),
            Box::new(m20241108_011153_create_locations::Migration),
            Box::new(m20241108_011410_create_rating::Migration),
            Box::new(m20241108_011927_create_raves::Migration),
            Box::new(m20241108_013020_create_expenses::Migration),
            Box::new(m20241109_224247_create_rave_artists::Migration),
            Box::new(m20241109_224816_create_expenses_rave_fk::Migration),
        ]
    }
}
