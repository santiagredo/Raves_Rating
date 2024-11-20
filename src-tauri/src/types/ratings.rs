use std::marker::PhantomData;

use models::entities::ratings::{Column, Entity, Model};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    config::get_config,
    utils::{Conditions, Core, Data, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn select_ratings(rating: Model, conditions: Conditions) -> Result<Vec<Model>, String> {
    let ratings = Ratings::<Core>::select_ratings(rating, conditions).await?;
    Ratings::<Data>::select_ratings(&get_config().await.db, ratings).await
}

#[derive(Debug, Default)]
pub struct Ratings<Phase = Core> {
    phase: PhantomData<Phase>,
    pub ratings_collection: Vec<Model>,
    pub conditions: Conditions,
}

impl Ratings<Core> {
    pub async fn select_ratings(
        rating: Model,
        conditions: Conditions,
    ) -> Result<Ratings<Data>, String> {
        let name = Utils::validate_empty_field(rating.name, "rating name");

        if rating.id == 0 && name.is_err() && !conditions.get_all {
            return Err(format!("Missing searching parameters"));
        }

        let id = rating.id;
        let name = name.unwrap_or_default();

        Ok(Ratings {
            phase: PhantomData::<Data>,
            ratings_collection: vec![Model { id, name }],
            conditions,
        })
    }
}

impl Ratings<Data> {
    pub async fn select_ratings(
        db: &DatabaseConnection,
        ratings: Self,
    ) -> Result<Vec<Model>, String> {
        let rating_model = match ratings.ratings_collection.get(0) {
            None => return Err(format!("No searching parameters in collection")),
            Some(val) => val.to_owned(),
        };

        let mut condition = Condition::all();

        if rating_model.id != 0 {
            condition = condition.add(Column::Id.eq(rating_model.id));
        }

        if !rating_model.name.is_empty() {
            condition = condition.add(Column::Name.eq(rating_model.name));
        }

        if !ratings.conditions.get_all && condition.len() < 1 {
            return Err(format!("No searching parameters"));
        };

        let query = match ratings.conditions.get_all {
            true => Entity::find(),
            _ => Entity::find().filter(condition),
        };

        match query.all(db).await {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => {
                if val.len() < 1 {
                    return Err(format!("No results found"));
                };

                Ok(val)
            }
        }
    }
}
