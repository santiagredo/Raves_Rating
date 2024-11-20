use chrono::NaiveDate;
use models::entities::{artists, expenses, locations, ratings, raves};
use sea_orm::{DbErr, FromQueryResult, RuntimeErr};
use serde::{Deserialize, Serialize};

pub struct Core;
pub struct Data;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conditions {
    pub get_upcoming: bool,
    pub get_previous: bool,
    pub get_all: bool,
    pub limit: i32,
    pub offset: i32,
    pub get_artists: bool,
    pub get_expenses: bool,
    pub get_locations: bool,
    pub get_ratings: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, FromQueryResult)]
pub struct RaveOverview {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
    pub location: String,
    pub rating: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RaveDetail {
    pub rave: raves::Model,
    pub location: locations::Model,
    pub rating: ratings::Model,
    pub artists: Vec<artists::Model>,
    pub expenses: Vec<expenses::Model>,
}

pub struct Utils;

impl Utils {
    pub fn validate_empty_field(field: String, field_name: &str) -> Result<String, String> {
        match field.trim().is_empty() {
            true => Err(format!("{field_name} cannot be empty")),
            false => Ok(field.to_lowercase()),
        }
    }

    pub fn get_error_type(err: DbErr) -> String {
        if let DbErr::Exec(RuntimeErr::SqlxError(sql_error)) = &err {
            let db_err = match sql_error.as_database_error() {
                None => return format!("SQL error without database details: {}", sql_error),
                Some(val) => val,
            };

            let err_code = match db_err.code() {
                None => return format!("SQL error without database details: {}", sql_error),
                Some(val) => val.to_string(),
            };

            match err_code.as_str() {
                "23505" => {
                    // Unique constraint violation (PostgreSQL)
                    "A record with this information already exists. Please try a different value."
                        .to_string();
                }
                "23503" => {
                    // Foreign key violation (PostgreSQL)
                    "One of the associated records could not be found. Please check your input and try again.".to_string();
                }
                "23502" => {
                    // Not null violation (PostgreSQL)
                    "A required field is missing. Please fill in all required fields and try again."
                        .to_string();
                }
                "23514" => {
                    // Check constraint violation (PostgreSQL)
                    "One or more values do not meet the required conditions. Please review and try again.".to_string();
                }
                _ => {
                    // Generic database error message for unhandled cases
                    "An unexpected database error occurred. Please try again later.".to_string();
                }
            }
        }

        match err {
            DbErr::Exec(runtime_err) => match runtime_err {
                RuntimeErr::Internal(msg) => format!("Internal runtime error: {}", msg),
                _ => format!("Unknown runtime error: {:?}", runtime_err),
            },
            DbErr::Query(query_err) => format!("Query error: {:?}", query_err),
            DbErr::Migration(migration_err) => format!("Migration error: {:?}", migration_err),
            DbErr::ConnectionAcquire(err) => format!("Connection acquisition error: {:?}", err),
            DbErr::Conn(conn_err) => format!("Connection error: {:?}", conn_err),
            DbErr::Type(type_err) => format!("Type error: {:?}", type_err),
            DbErr::Json(json_err) => format!("JSON error: {:?}", json_err),
            DbErr::RecordNotFound(msg) => format!("Record not found: {}", msg),
            DbErr::Custom(msg) => format!("Custom error: {}", msg),
            _ => format!("Unknown database error: {:?}", err),
        }
    }
}
