use std::sync::Arc;

use crate::{
    db::common::ConnectionPool,
    schema::messages,
    structs::{app_state::AppState, task::Task},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Identifiable,
    Selectable,
    AsChangeset,
    Insertable,
    Clone,
)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Option<i32>,
    pub content: String,
    pub creation_date: NaiveDateTime,
    pub author_id: i32,
}

impl Message {
    pub async fn send_task_to_worker(&self, conn: Arc<ConnectionPool>, app_state: AppState) {
        if let Some(id) = self.id {
            let task = Task {
                id,
                message: self.clone(),
                app_state: app_state.clone(),
                conn,
            };

            {
                let queue = app_state.worker_queues.get("gpt-4o").unwrap();
                let mut task_queue = queue.task_queue.lock().await;
                task_queue.insert(0, task);
                queue.notifier.notify_one();
            }
        }
    }
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = messages)]
pub struct InputMessage {
    pub content: String,
}
