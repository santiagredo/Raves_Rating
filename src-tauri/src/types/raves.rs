use std::marker::PhantomData;

use chrono::{Datelike, Local, NaiveDate};
use models::entities::{
    artists,
    expenses::{self},
    locations,
    prelude::{Expenses, RaveArtist},
    ratings,
    rave_artist::{self},
    raves::{self, Column, Entity, Model},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    JoinType, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    config::get_config,
    utils::{Conditions, Core, Data, RaveDetail, RaveOverview, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_raves(raves_collection: Vec<Model>) -> Result<i32, String> {
    let raves = Raves::<Core>::insert_raves(raves_collection).await?;
    Raves::<Data>::insert_raves(&get_config().await.db, raves).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_raves(rave: Model, conditions: Conditions) -> Result<Vec<Model>, String> {
    let raves = Raves::<Core>::select_raves(rave, conditions).await?;
    Raves::<Data>::select_raves(&get_config().await.db, raves).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_raves_overview(conditions: Conditions) -> Result<Vec<RaveOverview>, String> {
    let raves = Raves::<Core>::select_raves_overview(conditions).await?;
    Raves::<Data>::select_raves_overview(&get_config().await.db, raves).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_rave_detail(rave: Model) -> Result<RaveDetail, String> {
    let raves = Raves::<Core>::select_rave_detail(rave).await?;
    Raves::<Data>::select_rave_detail(&get_config().await.db, raves).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_raves(raves_collection: Vec<Model>) -> Result<u64, String> {
    let raves = Raves::<Core>::update_raves(raves_collection).await?;
    Raves::<Data>::update_raves(&get_config().await.db, raves).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_raves(raves_collection: Vec<Model>) -> Result<u64, String> {
    let raves = Raves::<Core>::delete_raves(raves_collection).await?;
    Raves::<Data>::delete_raves(&get_config().await.db, raves).await
}

#[derive(Debug, Default)]
pub struct Raves<Phase = Core> {
    phase: PhantomData<Phase>,
    pub raves_collection: Vec<Model>,
    pub conditions: Conditions,
}

impl Raves<Core> {
    async fn insert_raves(raves_collection: Vec<Model>) -> Result<Raves<Data>, String> {
        if raves_collection.len() < 1 {
            return Err(format!("Raves cannot be empty"));
        }

        let mut validated_raves: Vec<Model> = Vec::new();

        for (index, rave) in raves_collection.into_iter().enumerate() {
            let name = match Utils::validate_empty_field(rave.name, "Rave name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let location = match rave.location {
                0 => return Err(format!("Failure at index: {index}, Invalid location ID")),
                val => val,
            };

            let rating = match rave.rating {
                0 => return Err(format!("Failure at index: {index}, Invalid rating ID")),
                val => val,
            };

            let date = match rave.date == NaiveDate::default() {
                true => return Err(format!("Failure at index: {index}, Invalid rave date")),
                false => rave.date,
            };

            let validated_rave = Model {
                name,
                location,
                rating,
                date,
                ..Default::default()
            };

            validated_raves.push(validated_rave);
        }

        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: validated_raves,
            conditions: Conditions::default(),
        })
    }

    pub async fn select_raves(rave: Model, conditions: Conditions) -> Result<Raves<Data>, String> {
        let name = Utils::validate_empty_field(rave.name, "Rave name");

        let location = match rave.location {
            0 => Err(format!("Invalid location ID")),
            val => Ok(val),
        };

        let rating = match rave.rating {
            0 => Err(format!("Invalid rating ID")),
            val => Ok(val),
        };

        let date = match rave.date == NaiveDate::default() {
            true => Err(format!("Invalid rave date")),
            false => Ok(rave.date),
        };

        if rave.id == 0
            && name.is_err()
            && location.is_err()
            && rating.is_err()
            && date.is_err()
            && conditions.get_all == false
        {
            return Err(format!("Missing searching parameters"));
        }

        let id = rave.id;
        let name = name.unwrap_or_default();
        let location = location.unwrap_or_default();
        let rating = rating.unwrap_or_default();
        let date = date.unwrap_or_default();

        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: vec![Model {
                id,
                name,
                location,
                rating,
                date,
            }],
            conditions,
        })
    }

    pub async fn select_raves_overview(conditions: Conditions) -> Result<Raves<Data>, String> {
        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: Vec::new(),
            conditions,
        })
    }

    pub async fn select_rave_detail(rave: Model) -> Result<Raves<Data>, String> {
        if rave.id < 1 {
            return Err(format!("Invalid rave ID"));
        };

        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: vec![Model {
                id: rave.id,
                ..Default::default()
            }],
            conditions: Conditions::default(),
        })
    }

    pub async fn update_raves(raves: Vec<Model>) -> Result<Raves<Data>, String> {
        if raves.len() < 1 {
            return Err(format!("Raves cannot be empty"));
        }

        let mut validated_raves: Vec<Model> = Vec::new();

        for (index, rave) in raves.into_iter().enumerate() {
            let id = match rave.id {
                0 => return Err(format!("Failure at index: {index}, Rave ID cannot be 0")),
                val => val,
            };

            let name = match Utils::validate_empty_field(rave.name, "Rave name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let location = match rave.location {
                0 => return Err(format!("Failure at index: {index}, Invalid location ID")),
                val => val,
            };

            let rating = match rave.rating {
                0 => return Err(format!("Failure at index: {index}, Invalid rating ID")),
                val => val,
            };

            let date = match rave.date == NaiveDate::default() {
                true => return Err(format!("Failure at index: {index}, Invalid rave date")),
                false => rave.date,
            };

            let validated_rave = Model {
                id,
                name,
                location,
                rating,
                date,
            };

            validated_raves.push(validated_rave);
        }

        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: validated_raves,
            conditions: Conditions::default(),
        })
    }

    pub async fn delete_raves(raves: Vec<Model>) -> Result<Raves<Data>, String> {
        if raves.len() < 1 {
            return Err(format!("Raves cannot be empty"));
        }

        let mut validated_raves: Vec<Model> = Vec::new();

        for (index, rave) in raves.into_iter().enumerate() {
            let id = match rave.id {
                0 => return Err(format!("Failure at index: {index}, Rave ID cannot be 0")),
                val => val,
            };

            let validated_rave = Model {
                id,
                ..Default::default()
            };

            validated_raves.push(validated_rave);
        }

        Ok(Raves {
            phase: PhantomData::<Data>,
            raves_collection: validated_raves,
            conditions: Conditions::default(),
        })
    }
}

impl Raves<Data> {
    async fn insert_raves(db: &DatabaseConnection, raves: Self) -> Result<i32, String> {
        let mut active_model_raves = Vec::new();

        for rave_model in raves.raves_collection {
            let active_model_rave = raves::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(rave_model.name),
                location: ActiveValue::Set(rave_model.location),
                rating: ActiveValue::Set(rave_model.rating),
                date: ActiveValue::Set(rave_model.date),
            };

            active_model_raves.push(active_model_rave);
        }

        match Entity::insert_many(active_model_raves).exec(db).await {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => Ok(val.last_insert_id),
        }
    }

    pub async fn select_raves(db: &DatabaseConnection, raves: Self) -> Result<Vec<Model>, String> {
        let rave_model = match raves.raves_collection.get(0) {
            None => return Err(format!("No searching parameters in collection")),
            Some(val) => val.to_owned(),
        };

        let mut condition = Condition::all();

        if rave_model.id != 0 {
            condition = condition.add(Column::Id.eq(rave_model.id));
        }

        if !rave_model.name.trim().is_empty() {
            condition = condition.add(Column::Name.eq(rave_model.name));
        }

        if rave_model.location != 0 {
            condition = condition.add(Column::Location.eq(rave_model.location));
        }

        if rave_model.rating != 0 {
            condition = condition.add(Column::Rating.eq(rave_model.rating));
        }

        if rave_model.date != NaiveDate::default() {
            condition = condition.add(Column::Date.eq(rave_model.date));
        }

        if raves.conditions.get_upcoming {
            let current_date = Local::now();

            condition = condition.add(Column::Date.gt(NaiveDate::from_ymd_opt(
                current_date.year(),
                current_date.month(),
                current_date.day(),
            )))
        }

        if !raves.conditions.get_all && condition.len() < 1 {
            return Err(format!("No searching parameters"));
        };

        let mut query = match raves.conditions.get_all {
            true => Entity::find(),
            _ => Entity::find().filter(condition),
        };

        if raves.conditions.limit > 0 {
            query = query.limit(Some(raves.conditions.limit as u64))
        }

        if raves.conditions.offset > 0 {
            query = query.offset(Some(raves.conditions.offset as u64))
        }

        match query.all(db).await {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => {
                // if val.len() < 1 {
                //     return Err(format!("No results found"));
                // };

                Ok(val)
            }
        }
    }

    pub async fn select_raves_overview(
        db: &DatabaseConnection,
        raves: Self,
    ) -> Result<Vec<RaveOverview>, String> {
        let mut query = Entity::find();
        query = query.order_by(Column::Date, sea_orm::Order::Desc);

        if raves.conditions.get_upcoming {
            let current_date = Local::now();

            query = query.filter(Column::Date.gte(NaiveDate::from_ymd_opt(
                current_date.year(),
                current_date.month(),
                current_date.day(),
            )));

            query = query.order_by(Column::Date, sea_orm::Order::Asc);
        }

        if raves.conditions.get_previous {
            let current_date = Local::now();

            query = query.filter(Column::Date.lt(NaiveDate::from_ymd_opt(
                current_date.year(),
                current_date.month(),
                current_date.day(),
            )));
        }

        if raves.conditions.limit > 0 {
            query = query.limit(Some(raves.conditions.limit as u64))
        }

        if raves.conditions.offset > 0 {
            query = query.offset(Some(raves.conditions.offset as u64))
        }

        query = query.join_rev(
            JoinType::InnerJoin,
            locations::Entity::belongs_to(raves::Entity)
                .from(locations::Column::Id)
                .to(raves::Column::Location)
                .into(),
        );

        query = query.join_rev(
            JoinType::InnerJoin,
            ratings::Entity::belongs_to(raves::Entity)
                .from(ratings::Column::Id)
                .to(raves::Column::Rating)
                .into(),
        );

        match query
            .select_only()
            .column_as(raves::Column::Id, "id")
            .column_as(raves::Column::Name, "name")
            .column_as(raves::Column::Date, "date")
            .column_as(locations::Column::Name, "location")
            .column_as(ratings::Column::Name, "rating")
            .into_model::<RaveOverview>()
            .all(db)
            .await
        {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_rave_detail(
        db: &DatabaseConnection,
        raves: Self,
    ) -> Result<RaveDetail, String> {
        let rave = raves
            .raves_collection
            .get(0)
            .ok_or(format!("Missing rave"))?
            .to_owned();

        let rave = Entity::find_by_id(rave.id)
            .one(db)
            .await
            .map_err(Utils::get_error_type)?
            .ok_or("Rave not found")?;

        let location = rave
            .find_related(locations::Entity)
            .one(db)
            .await
            .map_err(Utils::get_error_type)?
            .ok_or("Location not found")?;

        let rating = rave
            .find_related(ratings::Entity)
            .one(db)
            .await
            .map_err(Utils::get_error_type)?
            .ok_or("Rating not found")?;

        let artists: Vec<artists::Model> = RaveArtist::find()
            .filter(rave_artist::Column::RaveId.eq(rave.id))
            .find_also_related(artists::Entity)
            .order_by(artists::Column::Name, sea_orm::Order::Asc)
            .all(db)
            .await
            .map_err(Utils::get_error_type)?
            .into_iter()
            .filter_map(|(_, artist)| artist)
            .collect();

        let expenses = Expenses::find()
            .filter(expenses::Column::RaveId.eq(rave.id))
            .all(db)
            .await
            .map_err(Utils::get_error_type)?;

        Ok(RaveDetail {
            rave,
            location,
            rating,
            artists,
            expenses,
        })
    }

    pub async fn update_raves(db: &DatabaseConnection, raves: Self) -> Result<u64, String> {
        let mut rows_affected: u64 = 0;

        for rave_model in raves.raves_collection {
            let mut rave = raves::ActiveModel {
                id: ActiveValue::Unchanged(rave_model.id),
                ..Default::default()
            };

            if !rave_model.name.is_empty() {
                rave.name = ActiveValue::set(rave_model.name)
            }

            if rave_model.location != 0 {
                rave.location = ActiveValue::set(rave_model.location)
            }

            if rave_model.rating != 0 {
                rave.rating = ActiveValue::set(rave_model.rating)
            }

            if rave_model.date != NaiveDate::default() {
                rave.date = ActiveValue::set(rave_model.date)
            }

            match rave.update(db).await {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(_) => rows_affected += 1,
            }
        }

        Ok(rows_affected)
    }

    pub async fn delete_raves(db: &DatabaseConnection, raves: Self) -> Result<u64, String> {
        let mut rows_affected = 0;

        let ids_collection: Vec<i32> = raves.raves_collection.into_iter().map(|x| x.id).collect();

        match rave_artist::Entity::delete_many()
            .filter(models::entities::rave_artist::Column::RaveId.is_in(ids_collection.clone()))
            .exec(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => rows_affected += val.rows_affected,
        };

        match expenses::Entity::delete_many()
            .filter(models::entities::expenses::Column::RaveId.is_in(ids_collection.clone()))
            .exec(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => rows_affected += val.rows_affected,
        };

        match Entity::delete_many()
            .filter(Column::Id.is_in(ids_collection))
            .exec(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => rows_affected += val.rows_affected,
        };

        Ok(rows_affected)
    }
}
