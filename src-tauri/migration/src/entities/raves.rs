//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "raves"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub location: i32,
    pub rating: i32,
    pub date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Location,
    Rating,
    Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Expenses,
    Locations,
    Ratings,
    RaveArtist,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def(),
            Self::Location => ColumnType::Integer.def(),
            Self::Rating => ColumnType::Integer.def(),
            Self::Date => ColumnType::Date.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Expenses => Entity::has_many(super::expenses::Entity).into(),
            Self::Locations => Entity::belongs_to(super::locations::Entity)
                .from(Column::Location)
                .to(super::locations::Column::Id)
                .into(),
            Self::Ratings => Entity::belongs_to(super::ratings::Entity)
                .from(Column::Rating)
                .to(super::ratings::Column::Id)
                .into(),
            Self::RaveArtist => Entity::has_many(super::rave_artist::Entity).into(),
        }
    }
}

impl Related<super::expenses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expenses.def()
    }
}

impl Related<super::locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Locations.def()
    }
}

impl Related<super::ratings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ratings.def()
    }
}

impl Related<super::rave_artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RaveArtist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
