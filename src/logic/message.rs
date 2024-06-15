use chrono::Utc;

use crate::db;
use crate::db::common::ConnectionPool;
use crate::models::message::{InputMessage, Message};

pub async fn get_message(conn: &ConnectionPool, id: i32) -> Message {
    db::message::get_message(conn, id).await
}

pub async fn get_messages(conn: &ConnectionPool) -> Vec<Message> {
    db::message::get_messages(conn).await
}

pub async fn get_messages_paginated(conn: &ConnectionPool, skip: u32, take: u32) -> Vec<Message> {
    db::message::get_messages_paginated(conn, skip, take).await
}

pub async fn delete_message<'a>(conn: &ConnectionPool, id: i32) -> bool {
    db::message::delete_message(conn, id).await
}

pub async fn create_message<'a>(conn: &ConnectionPool, new_message: InputMessage) -> Message {
    let message = Message {
        id: None,
        content: new_message.content,
        creation_date: Utc::now().naive_utc(),
        author_id: new_message.author_id, // TODO: This should be set automatically by looking at the request's author.
    };

    db::message::create_message(conn, message).await
}

pub async fn update_message<'a>(conn: &ConnectionPool, id: i32, message: InputMessage) -> Message {
    db::message::update_message(conn, id, message).await
}
