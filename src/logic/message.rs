use std::sync::Arc;

use chrono::Utc;

use crate::db;
use crate::db::common::ConnectionPool;
use crate::models::message::{InputMessage, Message};
use crate::structs::app_state::AppState;

use super::user::create_chatgpt_user_if_not_existing;

pub async fn get_message(conn: &ConnectionPool, id: i32) -> Message {
    db::message::get_message(conn, id).await
}

pub async fn get_messages(
    conn: &ConnectionPool,
    skip: Option<u32>,
    take: Option<u32>,
) -> Vec<Message> {
    db::message::get_messages(conn, skip, take).await
}

pub async fn delete_message(conn: &ConnectionPool, id: i32) -> bool {
    db::message::delete_message(conn, id).await
}

pub async fn create_message(
    conn: ConnectionPool,
    new_message: InputMessage,
    user_id: i32,
    app_state: &AppState,
) {
    let conn = Arc::new(conn);
    let message = Message {
        id: None,
        content: new_message.content,
        creation_date: Utc::now().naive_utc(),
        author_id: user_id,
    };

    let message = db::message::create_message(&conn, message).await;

    let _ = app_state.message_sender.send(message.clone());

    create_chatgpt_user_if_not_existing(&conn).await;
    message.send_task_to_worker(conn, app_state.clone()).await;
}

pub async fn update_message(
    conn: ConnectionPool,
    id: i32,
    message: InputMessage,
    app_state: &AppState,
) {
    let conn = Arc::new(conn);
    let message = db::message::update_message(&conn, id, message).await;

    let _ = app_state.message_sender.send(message.clone());

    create_chatgpt_user_if_not_existing(&conn).await;
    message.send_task_to_worker(conn, app_state.clone()).await;
}
