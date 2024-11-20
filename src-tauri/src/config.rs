use std::{env, str::FromStr};

use sea_orm::{Database, DatabaseConnection};

pub struct Config {
    pub db: DatabaseConnection,
}

fn get_env_val<T: FromStr>(key: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .expect(&format!("{key} not set"))
        .parse::<T>()
        .expect(&format!("Can't parse {key} to desired type"))
}

pub async fn load_settings() {
    dotenvy::dotenv().unwrap();

    let env_db_url: String = get_env_val("DATABASE_URL");

    Database::connect(&env_db_url).await.unwrap();
}

pub async fn get_config() -> Config {
    let env_db_url: String = get_env_val("DATABASE_URL");
    let db = Database::connect(&env_db_url).await.unwrap();

    Config { db }
}