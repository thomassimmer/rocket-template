use crate::db::common::ConnectionPool;
use crate::models::message::{InputMessage, Message};
use crate::schema::messages;
use diesel;
use diesel::prelude::*;

pub async fn get_messages(
    conn: &ConnectionPool,
    skip: Option<u32>,
    take: Option<u32>,
) -> Vec<Message> {
    let mut query = messages::table.select(Message::as_select()).into_boxed();

    if let Some(take) = take {
        query = query.limit(take as i64);
    }

    if let Some(skip) = skip {
        query = query.offset(skip as i64)
    }

    conn.run(move |c| query.load::<Message>(c).expect("Error loading messages"))
        .await
}

pub async fn get_message(conn: &ConnectionPool, message_id: i32) -> Message {
    conn.run(move |c| {
        messages::table
            .filter(messages::id.eq(message_id))
            .select(Message::as_select())
            .first::<Message>(c)
            .expect("Error loading Message")
    })
    .await
}

pub async fn create_message(conn: &ConnectionPool, message: Message) -> Message {
    conn.run(|c| {
        diesel::insert_into(messages::table)
            .values(message)
            .returning(Message::as_returning())
            .get_result(c)
            .expect("Error saving new message")
    })
    .await
}

pub async fn update_message(
    conn: &ConnectionPool,
    message_id: i32,
    message: InputMessage,
) -> Message {
    conn.run(move |c| {
        diesel::update(messages::table.filter(messages::id.eq(message_id)))
            .set(&message)
            .returning(Message::as_returning())
            .get_result(c)
            .unwrap_or_else(|_| panic!("Unable to update message {:?}", message_id))
    })
    .await
}

pub async fn delete_message(conn: &ConnectionPool, message_id: i32) -> bool {
    let num_deleted = conn
        .run(move |c| {
            diesel::delete(messages::table.filter(messages::id.eq(message_id)))
                .execute(c)
                .expect("Error deleting message")
        })
        .await;

    matches!(num_deleted, 1)
}
