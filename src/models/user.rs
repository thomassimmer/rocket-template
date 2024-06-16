use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Serialize, Deserialize, Debug, Identifiable, Selectable, AsChangeset, Insertable,
)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub creation_date: NaiveDateTime,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
pub struct InputUserCreation {
    pub user_name: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
pub struct InputUserUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
