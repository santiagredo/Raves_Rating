use std::marker::PhantomData;

use models::entities::locations::{self, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::{
    config::get_config,
    utils::{Conditions, Core, Data, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_locations(locations_collection: Vec<Model>) -> Result<i32, String> {
    let locations = Locations::<Core>::insert_locations(locations_collection).await?;
    Locations::<Data>::insert_locations(&get_config().await.db, locations).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_locations(
    location: Model,
    conditions: Conditions,
) -> Result<Vec<Model>, String> {
    let locations = Locations::<Core>::select_locations(location, conditions).await?;
    Locations::<Data>::select_locations(&get_config().await.db, locations).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_locations(locations_collection: Vec<Model>) -> Result<u64, String> {
    let locations = Locations::<Core>::update_locations(locations_collection).await?;
    Locations::<Data>::update_locations(&get_config().await.db, locations).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_locations(locations_collection: Vec<Model>) -> Result<u64, String> {
    let locations = Locations::<Core>::delete_locations(locations_collection).await?;
    Locations::<Data>::delete_locations(&get_config().await.db, locations).await
}

#[derive(Debug, Default)]
pub struct Locations<Phase = Core> {
    phase: PhantomData<Phase>,
    pub locations_collection: Vec<Model>,
    pub conditions: Conditions,
}

impl Locations<Core> {
    async fn insert_locations(locations_collection: Vec<Model>) -> Result<Locations<Data>, String> {
        if locations_collection.len() < 1 {
            return Err(format!("Locations cannot be empty"));
        }

        let mut validated_locations: Vec<Model> = Vec::new();

        for (index, location) in locations_collection.into_iter().enumerate() {
            let name = match Utils::validate_empty_field(location.name, "Location name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let address = match Utils::validate_empty_field(location.address, "Location address") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let capacity = match Utils::validate_empty_field(location.capacity, "Location capacity")
            {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let validated_location = Model {
                name,
                address,
                capacity,
                ..Default::default()
            };

            validated_locations.push(validated_location);
        }

        Ok(Locations {
            phase: PhantomData::<Data>,
            locations_collection: validated_locations,
            conditions: Conditions::default(),
        })
    }

    pub async fn select_locations(
        location: Model,
        conditions: Conditions,
    ) -> Result<Locations<Data>, String> {
        let name = Utils::validate_empty_field(location.name, "Location name");
        let address = Utils::validate_empty_field(location.address, "Location address");
        let capacity = Utils::validate_empty_field(location.capacity, "Location capacity");

        if location.id == 0
            && name.is_err()
            && address.is_err()
            && capacity.is_err()
            && conditions.get_all == false
        {
            return Err(format!("Missing searching parameters"));
        }

        let id = location.id;
        let name = name.unwrap_or_default();
        let address = address.unwrap_or_default();
        let capacity = capacity.unwrap_or_default();

        Ok(Locations {
            phase: PhantomData::<Data>,
            locations_collection: vec![Model {
                id,
                name,
                address,
                capacity,
            }],
            conditions,
        })
    }

    pub async fn update_locations(locations: Vec<Model>) -> Result<Locations<Data>, String> {
        if locations.len() < 1 {
            return Err(format!("Locations cannot be empty"));
        }

        let mut validated_locations: Vec<Model> = Vec::new();

        for (index, location) in locations.into_iter().enumerate() {
            let id = match location.id {
                0 => {
                    return Err(format!(
                        "Failure at index: {index}, Location ID cannot be 0"
                    ))
                }
                val => val,
            };

            let name = match Utils::validate_empty_field(location.name, "Location name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let address = match Utils::validate_empty_field(location.address, "Location address") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let capacity = match Utils::validate_empty_field(location.capacity, "Location capacity")
            {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let validated_location = Model {
                id,
                name,
                address,
                capacity,
            };

            validated_locations.push(validated_location);
        }

        Ok(Locations {
            phase: PhantomData::<Data>,
            locations_collection: validated_locations,
            conditions: Conditions::default(),
        })
    }

    pub async fn delete_locations(locations: Vec<Model>) -> Result<Locations<Data>, String> {
        if locations.len() < 1 {
            return Err(format!("Locations cannot be empty"));
        }

        let mut validated_locations: Vec<Model> = Vec::new();

        for (index, location) in locations.into_iter().enumerate() {
            let id = match location.id {
                0 => {
                    return Err(format!(
                        "Failure at index: {index}, Location ID cannot be 0"
                    ))
                }
                val => val,
            };

            let validated_location = Model {
                id,
                ..Default::default()
            };

            validated_locations.push(validated_location);
        }

        Ok(Locations {
            phase: PhantomData::<Data>,
            locations_collection: validated_locations,
            conditions: Conditions::default(),
        })
    }
}

impl Locations<Data> {
    async fn insert_locations(db: &DatabaseConnection, locations: Self) -> Result<i32, String> {
        let mut active_model_locations = Vec::new();

        for location_model in locations.locations_collection {
            let active_model_location = locations::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(location_model.name),
                address: ActiveValue::Set(location_model.address),
                capacity: ActiveValue::Set(location_model.capacity),
            };

            active_model_locations.push(active_model_location);
        }

        match Entity::insert_many(active_model_locations).exec(db).await {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => Ok(val.last_insert_id),
        }
    }

    pub async fn select_locations(
        db: &DatabaseConnection,
        locations: Self,
    ) -> Result<Vec<Model>, String> {
        let location_model = match locations.locations_collection.get(0) {
            None => return Err(format!("No searching parameters in collection")),
            Some(val) => val.to_owned(),
        };

        let mut condition = Condition::all();

        if location_model.id != 0 {
            condition = condition.add(Column::Id.eq(location_model.id));
        }

        if !location_model.name.is_empty() {
            condition = condition.add(Column::Name.eq(location_model.name));
        }

        if !location_model.address.is_empty() {
            condition = condition.add(Column::Address.eq(location_model.address));
        }

        if !location_model.capacity.is_empty() {
            condition = condition.add(Column::Capacity.eq(location_model.capacity));
        }

        if !locations.conditions.get_all && condition.len() < 1 {
            return Err(format!("No searching parameters"));
        };

        let mut query = match locations.conditions.get_all {
            true => Entity::find(),
            _ => Entity::find().filter(condition),
        };

        query = query.order_by(Column::Name, sea_orm::Order::Asc);

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

    pub async fn update_locations(db: &DatabaseConnection, locations: Self) -> Result<u64, String> {
        let mut rows_affected: u64 = 0;

        for location_model in locations.locations_collection {
            let mut location = locations::ActiveModel {
                id: ActiveValue::Unchanged(location_model.id),
                ..Default::default()
            };

            if !location_model.name.is_empty() {
                location.name = ActiveValue::set(location_model.name)
            }

            if !location_model.address.is_empty() {
                location.address = ActiveValue::set(location_model.address)
            }

            if !location_model.capacity.is_empty() {
                location.capacity = ActiveValue::set(location_model.capacity)
            }

            match location.update(db).await {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(_) => rows_affected += 1,
            }
        }

        Ok(rows_affected)
    }

    pub async fn delete_locations(db: &DatabaseConnection, locations: Self) -> Result<u64, String> {
        let ids_collection: Vec<i32> = locations
            .locations_collection
            .into_iter()
            .map(|x| x.id)
            .collect();

        match Entity::delete_many()
            .filter(Column::Id.is_in(ids_collection))
            .exec(db)
            .await
        {
            Err(err) => Err(Utils::get_error_type(err)),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
