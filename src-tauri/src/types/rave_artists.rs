use std::marker::PhantomData;

use models::entities::rave_artist::{self, Column, Entity, Model};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    config::get_config,
    utils::{Core, Data, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_rave_artists(relations_collection: Vec<Model>) -> Result<i32, String> {
    let relations = RaveArtists::<Core>::insert_rave_artists(relations_collection).await?;
    RaveArtists::<Data>::insert_rave_artists(&get_config().await.db, relations).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_rave_artists(relations_collection: Vec<Model>) -> Result<u64, String> {
    let relations = RaveArtists::<Core>::delete_rave_artists(relations_collection).await?;
    RaveArtists::<Data>::delete_rave_artists(&get_config().await.db, relations).await
}

#[derive(Debug, Default)]
pub struct RaveArtists<Phase = Core> {
    phase: PhantomData<Phase>,
    pub relations_collection: Vec<Model>,
}

impl RaveArtists<Core> {
    async fn insert_rave_artists(
        relations_collection: Vec<Model>,
    ) -> Result<RaveArtists<Data>, String> {
        if relations_collection.len() < 1 {
            return Err(format!("Rave Artists relation collection cannot be empty"));
        }

        let mut validated_relations: Vec<Model> = Vec::new();

        for (index, relation) in relations_collection.into_iter().enumerate() {
            let rave_id = match relation.rave_id {
                0 => return Err(format!("Failure at index: {index}, Rave ID cannot be 0")),
                val => val,
            };

            let artist_id = match relation.artist_id {
                0 => return Err(format!("Failure at index: {index}, Artist ID cannot be 0")),
                val => val,
            };

            let validated_relation = Model {
                id: 0,
                rave_id,
                artist_id,
            };

            validated_relations.push(validated_relation);
        }

        Ok(RaveArtists {
            phase: PhantomData::<Data>,
            relations_collection: validated_relations,
        })
    }

    async fn delete_rave_artists(
        relations_collection: Vec<Model>,
    ) -> Result<RaveArtists<Data>, String> {
        if relations_collection.len() < 1 {
            return Err(format!("Rave Artists relation collection cannot be empty"));
        }

        let mut validated_relations: Vec<Model> = Vec::new();

        for (index, relation) in relations_collection.into_iter().enumerate() {
            let rave_id = match relation.rave_id {
                0 => return Err(format!("Failure at index: {index}, Rave ID cannot be 0")),
                val => val,
            };

            let artist_id = match relation.artist_id {
                0 => return Err(format!("Failure at index: {index}, Artist ID cannot be 0")),
                val => val,
            };

            let validated_relation = Model {
                id: 0,
                rave_id,
                artist_id,
            };

            validated_relations.push(validated_relation);
        }

        Ok(RaveArtists {
            phase: PhantomData::<Data>,
            relations_collection: validated_relations,
        })
    }
}

impl RaveArtists<Data> {
    async fn insert_rave_artists(db: &DatabaseConnection, relations: Self) -> Result<i32, String> {
        let mut active_model_rave_artist_relations = Vec::new();

        for relation in relations.relations_collection {
            match Entity::find()
                .filter(
                    Condition::all()
                        .add(Column::RaveId.eq(relation.rave_id.clone()))
                        .add(Column::ArtistId.eq(relation.artist_id.clone())),
                )
                .all(db)
                .await
            {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(val) => {
                    if val.len() > 0 {
                        continue;
                    }
                }
            };

            let active_model_rave_artist_relation = rave_artist::ActiveModel {
                id: ActiveValue::NotSet,
                rave_id: ActiveValue::Set(relation.rave_id),
                artist_id: ActiveValue::Set(relation.artist_id),
            };

            active_model_rave_artist_relations.push(active_model_rave_artist_relation);
        }

        let result = match Entity::insert_many(active_model_rave_artist_relations)
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

    async fn delete_rave_artists(db: &DatabaseConnection, relations: Self) -> Result<u64, String> {
        let mut rows_affected = 0;

        for relation in relations.relations_collection {
            match rave_artist::Entity::delete_many()
                .filter(models::entities::rave_artist::Column::RaveId.eq(relation.rave_id))
                .filter(models::entities::rave_artist::Column::ArtistId.eq(relation.artist_id))
                .exec(db)
                .await
            {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(val) => rows_affected += val.rows_affected,
            }
        }

        Ok(rows_affected)
    }
}
