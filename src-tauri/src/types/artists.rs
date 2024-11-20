use std::marker::PhantomData;

use models::entities::{
    artists::{self, Column, Entity, Model},
    rave_artist,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::{
    config::get_config,
    utils::{Conditions, Core, Data, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_artist(artist: Model) -> Result<Model, String> {
    let artist = Artists::<Core>::insert_artist(artist).await?;
    Artists::<Data>::insert_artist(&get_config().await.db, artist).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_artists(artists_collection: Vec<Model>) -> Result<i32, String> {
    let artists = Artists::<Core>::insert_artists(artists_collection).await?;
    Artists::<Data>::insert_artists(&get_config().await.db, artists).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_artists(artist: Model, conditions: Conditions) -> Result<Vec<Model>, String> {
    let artists = Artists::<Core>::select_artists(artist, conditions).await?;
    Artists::<Data>::select_artists(&get_config().await.db, artists).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_artists(artists_collection: Vec<Model>) -> Result<u64, String> {
    let artists = Artists::<Core>::update_artists(artists_collection).await?;
    Artists::<Data>::update_artists(&get_config().await.db, artists).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_artists(artists_collection: Vec<Model>) -> Result<u64, String> {
    let artists = Artists::<Core>::delete_artists(artists_collection).await?;
    Artists::<Data>::delete_artists(&get_config().await.db, artists).await
}

#[derive(Debug, Default)]
pub struct Artists<Phase = Core> {
    phase: PhantomData<Phase>,
    pub artists_collection: Vec<Model>,
    pub conditions: Conditions,
}

impl Artists<Core> {
    async fn insert_artist(artist: Model) -> Result<Artists<Data>, String> {
        let name = match Utils::validate_empty_field(artist.name, "Artist name") {
            Err(err) => return Err(err),
            Ok(val) => val,
        };
        let country = match Utils::validate_empty_field(artist.country, "Artist country") {
            Err(err) => return Err(err),
            Ok(val) => val,
        };

        let validated_artist = Model {
            name,
            country,
            ..Default::default()
        };

        Ok(Artists {
            phase: PhantomData::<Data>,
            artists_collection: vec![validated_artist],
            conditions: Conditions::default(),
        })
    }

    async fn insert_artists(artists_collection: Vec<Model>) -> Result<Artists<Data>, String> {
        if artists_collection.len() < 1 {
            return Err(format!("Artists cannot be empty"));
        }

        let mut validated_artists: Vec<Model> = Vec::new();

        for (index, artist) in artists_collection.into_iter().enumerate() {
            let name = match Utils::validate_empty_field(artist.name, "Artist name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };
            let country = match Utils::validate_empty_field(artist.country, "Artist country") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let validated_artist = Model {
                name,
                country,
                ..Default::default()
            };

            validated_artists.push(validated_artist);
        }

        Ok(Artists {
            phase: PhantomData::<Data>,
            artists_collection: validated_artists,
            conditions: Conditions::default(),
        })
    }

    pub async fn select_artists(
        artist: Model,
        conditions: Conditions,
    ) -> Result<Artists<Data>, String> {
        let name = Utils::validate_empty_field(artist.name, "Artist name");
        let country = Utils::validate_empty_field(artist.country, "Artist country");

        if artist.id == 0 && name.is_err() && country.is_err() && conditions.get_all == false {
            return Err(format!("Missing searching parameters"));
        }

        let id = artist.id;
        let name = name.unwrap_or_default();
        let country = country.unwrap_or_default();

        Ok(Artists {
            phase: PhantomData::<Data>,
            artists_collection: vec![Model { id, name, country }],
            conditions,
        })
    }

    pub async fn update_artists(artists: Vec<Model>) -> Result<Artists<Data>, String> {
        if artists.len() < 1 {
            return Err(format!("Artists cannot be empty"));
        }

        let mut validated_artists: Vec<Model> = Vec::new();

        for (index, artist) in artists.into_iter().enumerate() {
            let id = match artist.id {
                0 => return Err(format!("Failure at index: {index}, Artist ID cannot be 0")),
                val => val,
            };

            let name = match Utils::validate_empty_field(artist.name, "Artist name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };
            let country = match Utils::validate_empty_field(artist.country, "Artist country") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let validated_artist = Model { id, name, country };

            validated_artists.push(validated_artist);
        }

        Ok(Artists {
            phase: PhantomData::<Data>,
            artists_collection: validated_artists,
            conditions: Conditions::default(),
        })
    }

    pub async fn delete_artists(artists: Vec<Model>) -> Result<Artists<Data>, String> {
        if artists.len() < 1 {
            return Err(format!("Artists cannot be empty"));
        }

        let mut validated_artists: Vec<Model> = Vec::new();

        for (index, artist) in artists.into_iter().enumerate() {
            let id = match artist.id {
                0 => return Err(format!("Failure at index: {index}, Artist ID cannot be 0")),
                val => val,
            };

            let validated_artist = Model {
                id,
                ..Default::default()
            };

            validated_artists.push(validated_artist);
        }

        Ok(Artists {
            phase: PhantomData::<Data>,
            artists_collection: validated_artists,
            conditions: Conditions::default(),
        })
    }
}

impl Artists<Data> {
    async fn insert_artist(db: &DatabaseConnection, artists: Self) -> Result<Model, String> {
        let artist_model = artists
            .artists_collection
            .get(0)
            .ok_or(format!("Artist cannot be empty"))?
            .to_owned();

        match Entity::find()
            .filter(Condition::all().add(Column::Name.eq(&artist_model.name)))
            .one(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => {
                if let Some(model) = val {
                    return Ok(model);
                }
            }
        }

        let active_model_artist = artists::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(artist_model.name),
            country: ActiveValue::Set(artist_model.country),
        };

        match Entity::insert(active_model_artist)
            .exec_with_returning(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => Ok(val),
        }
    }

    async fn insert_artists(db: &DatabaseConnection, artists: Self) -> Result<i32, String> {
        let mut active_model_artists = Vec::new();

        for artist_model in artists.artists_collection {
            match Entity::find()
                .filter(Condition::all().add(Column::Name.eq(&artist_model.name)))
                .all(db)
                .await
            {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(val) => {
                    if val.len() > 0 {
                        continue;
                    }
                }
            }

            let active_model_artist = artists::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(artist_model.name),
                country: ActiveValue::Set(artist_model.country),
            };

            active_model_artists.push(active_model_artist);
        }

        let result = match Entity::insert_many(active_model_artists)
            .on_empty_do_nothing()
            .exec(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => val,
        };

        match result {
            sea_orm::TryInsertResult::Conflicted => Ok(0),
            sea_orm::TryInsertResult::Empty => Ok(0),
            sea_orm::TryInsertResult::Inserted(val) => Ok(val.last_insert_id),
        }
    }

    pub async fn select_artists(
        db: &DatabaseConnection,
        artists: Self,
    ) -> Result<Vec<Model>, String> {
        let artist_model = match artists.artists_collection.get(0) {
            None => return Err(format!("No searching parameters in collection")),
            Some(val) => val.to_owned(),
        };

        let mut condition = Condition::all();

        if artist_model.id != 0 {
            condition = condition.add(Column::Id.eq(artist_model.id));
        }

        if !artist_model.name.is_empty() {
            condition = condition.add(Column::Name.eq(artist_model.name));
        }

        if !artist_model.country.is_empty() {
            condition = condition.add(Column::Country.eq(artist_model.country));
        }

        if !artists.conditions.get_all && condition.len() < 1 {
            return Err(format!("No searching parameters"));
        };

        let mut query = match artists.conditions.get_all {
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

    pub async fn update_artists(db: &DatabaseConnection, artists: Self) -> Result<u64, String> {
        let mut rows_affected: u64 = 0;

        for artist_model in artists.artists_collection {
            let mut artist = artists::ActiveModel {
                id: ActiveValue::Unchanged(artist_model.id),
                ..Default::default()
            };

            if !artist_model.name.is_empty() {
                artist.name = ActiveValue::set(artist_model.name)
            }

            if !artist_model.country.is_empty() {
                artist.country = ActiveValue::set(artist_model.country)
            }

            match artist.update(db).await {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(_) => rows_affected += 1,
            }
        }

        Ok(rows_affected)
    }

    pub async fn delete_artists(db: &DatabaseConnection, artists: Self) -> Result<u64, String> {
        let mut rows_affected: u64 = 0;

        let ids_collection: Vec<i32> = artists
            .artists_collection
            .into_iter()
            .map(|x| x.id)
            .collect();

        match rave_artist::Entity::delete_many()
            .filter(rave_artist::Column::ArtistId.is_in(ids_collection.clone()))
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
