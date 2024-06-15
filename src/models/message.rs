use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Serialize, Deserialize, Debug, Identifiable, Selectable, AsChangeset, Insertable,
)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Option<i32>,
    pub content: String,
    pub creation_date: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = messages)]
pub struct InputMessage {
    pub content: String,
    pub author_id: i32, // TODO: This should be set automatically by looking at the request's author.
}
