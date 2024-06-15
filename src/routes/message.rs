use rocket::{delete, get, post, put, routes};

use crate::db::common::ConnectionPool;
use crate::logic;
use crate::models::message::{InputMessage, Message};
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_messages,
        get_message,
        get_messages_paginated,
        delete_message,
        create_message,
        update_message
    ]
}

#[get("/<id>")]
pub async fn get_message(conn: ConnectionPool, id: i32) -> Json<Message> {
    let message = logic::message::get_message(&conn, id).await;
    Json(message)
}

#[get("/")]
pub async fn get_messages(conn: ConnectionPool) -> Json<Vec<Message>> {
    let messagess = logic::message::get_messages(&conn).await;
    Json(messagess)
}

#[get("/?<skip>&<take>")]
pub async fn get_messages_paginated(
    conn: ConnectionPool,
    skip: u32,
    take: u32,
) -> Json<Vec<Message>> {
    let messagess = logic::message::get_messages_paginated(&conn, skip, take).await;
    Json(messagess)
}

#[delete("/<id>")]
pub async fn delete_message(conn: ConnectionPool, id: i32) -> String {
    logic::message::delete_message(&conn, id).await;
    "Ok".to_string()
}

#[post("/", format = "json", data = "<message>")]
pub async fn create_message(conn: ConnectionPool, message: Json<InputMessage>) -> Json<Message> {
    let message = logic::message::create_message(&conn, message.into_inner()).await;
    Json(message)
}

#[put("/<id>", format = "json", data = "<message>")]
pub async fn update_message(
    conn: ConnectionPool,
    id: i32,
    message: Json<InputMessage>,
) -> Json<Message> {
    let message = logic::message::update_message(&conn, id, message.into_inner()).await;
    Json(message)
}
