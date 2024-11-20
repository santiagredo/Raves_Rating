use std::marker::PhantomData;

use models::entities::expenses::{self, Column, Entity, Model};
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection,
    EntityTrait, QueryFilter,
};

use crate::{
    config::get_config,
    utils::{Core, Data, Utils},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_expenses(expenses_collection: Vec<Model>) -> Result<i32, String> {
    let expenses = Expenses::<Core>::insert_expenses(expenses_collection).await?;
    Expenses::<Data>::insert_expenses(&get_config().await.db, expenses).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_expenses(
    expense: Model,
    handle_all: Option<bool>,
) -> Result<Vec<Model>, String> {
    let expenses = Expenses::<Core>::select_expenses(expense, handle_all).await?;
    Expenses::<Data>::select_expenses(&get_config().await.db, expenses).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_expenses(expenses_collection: Vec<Model>) -> Result<u64, String> {
    let expenses = Expenses::<Core>::update_expenses(expenses_collection).await?;
    Expenses::<Data>::update_expenses(&get_config().await.db, expenses).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_expenses(expenses_collection: Vec<Model>) -> Result<u64, String> {
    let expenses = Expenses::<Core>::delete_expenses(expenses_collection).await?;
    Expenses::<Data>::delete_expenses(&get_config().await.db, expenses).await
}

#[derive(Debug, Default)]
pub struct Expenses<Phase = Core> {
    phase: PhantomData<Phase>,
    pub expenses_collection: Vec<Model>,
    pub handle_all: Option<bool>,
}

impl Expenses<Core> {
    async fn insert_expenses(expenses_collection: Vec<Model>) -> Result<Expenses<Data>, String> {
        if expenses_collection.len() < 1 {
            return Err(format!("Expenses cannot be empty"));
        }

        let mut validated_expenses: Vec<Model> = Vec::new();

        for (index, expense) in expenses_collection.into_iter().enumerate() {
            let name = match Utils::validate_empty_field(expense.name, "Expense name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let price = match expense.price {
                val if val <= Decimal::new(0, 0) => {
                    return Err(format!(
                        "Failure at index: {index}, Price cannot be negative or zero"
                    ))
                }
                val => val,
            };

            let rave_id = match expense.rave_id {
                0 => {
                    return Err(format!(
                        "Failure at index: {index}, Rave ID cannot be empty"
                    ))
                }
                val => val,
            };

            let validated_expense = Model {
                name,
                price,
                rave_id,
                ..Default::default()
            };

            validated_expenses.push(validated_expense);
        }

        Ok(Expenses {
            phase: PhantomData::<Data>,
            expenses_collection: validated_expenses,
            handle_all: None,
        })
    }

    pub async fn select_expenses(
        expense: Model,
        handle_all: Option<bool>,
    ) -> Result<Expenses<Data>, String> {
        let name = Utils::validate_empty_field(expense.name, "Expense name");
        let price = match expense.price {
            val if val <= Decimal::new(0, 0) => Err(format!("Price cannot be negative or zero")),
            val => Ok(val),
        };

        if expense.id == 0
            && name.is_err()
            && price.is_err()
            && handle_all.unwrap_or(false) == false
        {
            return Err(format!("Missing searching parameters"));
        }

        let id = expense.id;
        let name = name.unwrap_or_default();
        let price = price.unwrap_or_default();

        Ok(Expenses {
            phase: PhantomData::<Data>,
            expenses_collection: vec![Model {
                id,
                name,
                price,
                ..Default::default()
            }],
            handle_all,
        })
    }

    pub async fn update_expenses(expenses: Vec<Model>) -> Result<Expenses<Data>, String> {
        if expenses.len() < 1 {
            return Err(format!("Expenses cannot be empty"));
        }

        let mut validated_expenses: Vec<Model> = Vec::new();

        for (index, expense) in expenses.into_iter().enumerate() {
            let id = match expense.id {
                0 => return Err(format!("Failure at index: {index}, Expense ID cannot be 0")),
                val => val,
            };

            let name = match Utils::validate_empty_field(expense.name, "Expense name") {
                Err(err) => return Err(format!("Failure at index: {index}, {err}")),
                Ok(val) => val,
            };

            let price = match expense.price {
                val if val <= Decimal::new(0, 0) => {
                    return Err(format!(
                        "Failure at index: {index}, Price cannot be negative or zero"
                    ))
                }
                val => val,
            };

            let validated_expense = Model {
                id,
                name,
                price,
                ..Default::default()
            };

            validated_expenses.push(validated_expense);
        }

        Ok(Expenses {
            phase: PhantomData::<Data>,
            expenses_collection: validated_expenses,
            handle_all: None,
        })
    }

    pub async fn delete_expenses(expenses: Vec<Model>) -> Result<Expenses<Data>, String> {
        if expenses.len() < 1 {
            return Err(format!("Expenses cannot be empty"));
        }

        let mut validated_expenses: Vec<Model> = Vec::new();

        for (index, expense) in expenses.into_iter().enumerate() {
            let id = match expense.id {
                0 => return Err(format!("Failure at index: {index}, Expense ID cannot be 0")),
                val => val,
            };

            let validated_expense = Model {
                id,
                ..Default::default()
            };

            validated_expenses.push(validated_expense);
        }

        Ok(Expenses {
            phase: PhantomData::<Data>,
            expenses_collection: validated_expenses,
            handle_all: None,
        })
    }
}

impl Expenses<Data> {
    async fn insert_expenses(db: &DatabaseConnection, expenses: Self) -> Result<i32, String> {
        let mut active_model_expenses = Vec::new();

        for expense_model in expenses.expenses_collection {
            let active_model_expense = expenses::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(expense_model.name),
                price: ActiveValue::Set(expense_model.price),
                rave_id: ActiveValue::Set(expense_model.rave_id),
            };

            active_model_expenses.push(active_model_expense);
        }

        let result = match Entity::insert_many(active_model_expenses)
            .on_empty_do_nothing()
            .exec(db)
            .await
        {
            Err(err) => return Err(Utils::get_error_type(err)),
            Ok(val) => val,
        };

        match result {
            sea_orm::TryInsertResult::Empty => Ok(0),
            sea_orm::TryInsertResult::Conflicted => Ok(0),
            sea_orm::TryInsertResult::Inserted(val) => Ok(val.last_insert_id),
        }
    }

    pub async fn select_expenses(
        db: &DatabaseConnection,
        expenses: Self,
    ) -> Result<Vec<Model>, String> {
        let expense_model = match expenses.expenses_collection.get(0) {
            None => return Err(format!("No searching parameters in collection")),
            Some(val) => val.to_owned(),
        };

        let mut condition = Condition::all();

        if expense_model.id != 0 {
            condition = condition.add(Column::Id.eq(expense_model.id));
        }

        if !expense_model.name.is_empty() {
            condition = condition.add(Column::Name.eq(expense_model.name));
        }

        if expense_model.price > Decimal::new(0, 0) {
            condition = condition.add(Column::Price.eq(expense_model.price));
        }

        if !expenses.handle_all.unwrap_or(false) && condition.len() < 1 {
            return Err(format!("No searching parameters"));
        };

        let query = match expenses.handle_all {
            Some(true) => Entity::find(),
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

    pub async fn update_expenses(db: &DatabaseConnection, expenses: Self) -> Result<u64, String> {
        let mut rows_affected: u64 = 0;

        for expense_model in expenses.expenses_collection {
            let mut expense = expenses::ActiveModel {
                id: ActiveValue::Unchanged(expense_model.id),
                ..Default::default()
            };

            if !expense_model.name.is_empty() {
                expense.name = ActiveValue::set(expense_model.name)
            }

            if expense_model.price > Decimal::new(0, 0) {
                expense.price = ActiveValue::set(expense_model.price)
            }

            match expense.update(db).await {
                Err(err) => return Err(Utils::get_error_type(err)),
                Ok(_) => rows_affected += 1,
            }
        }

        Ok(rows_affected)
    }

    pub async fn delete_expenses(db: &DatabaseConnection, expenses: Self) -> Result<u64, String> {
        let ids_collection: Vec<i32> = expenses
            .expenses_collection
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
