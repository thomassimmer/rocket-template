use rocket::{delete, get, post, put, routes};

use crate::db::common::ConnectionPool;
use crate::logic;
use crate::models::message::{InputMessage, Message};
use crate::structs::jwt::JWT;
use crate::structs::response::NetworkResponse;
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
pub async fn get_message(
    conn: ConnectionPool,
    id: i32,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Message>, NetworkResponse> {
    let _key = key?;
    let message = logic::message::get_message(&conn, id).await;
    Ok(Json(message))
}

#[get("/")]
pub async fn get_messages(
    conn: ConnectionPool,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Vec<Message>>, NetworkResponse> {
    let _key = key?;
    let messages = logic::message::get_messages(&conn).await;
    Ok(Json(messages))
}

#[get("/?<skip>&<take>")]
pub async fn get_messages_paginated(
    conn: ConnectionPool,
    skip: u32,
    take: u32,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Vec<Message>>, NetworkResponse> {
    let _key = key?;
    let messages = logic::message::get_messages_paginated(&conn, skip, take).await;
    Ok(Json(messages))
}

#[delete("/<id>")]
pub async fn delete_message(
    conn: ConnectionPool,
    id: i32,
    key: Result<JWT, NetworkResponse>,
) -> Result<String, NetworkResponse> {
    let _key = key?;
    logic::message::delete_message(&conn, id).await;
    Ok("Ok".to_string())
}

#[post("/", format = "json", data = "<message>")]
pub async fn create_message(
    conn: ConnectionPool,
    message: Json<InputMessage>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Message>, NetworkResponse> {
    let key = key?;
    let user_id = key.claims.subject_id;
    let message = logic::message::create_message(&conn, message.into_inner(), user_id).await;
    Ok(Json(message))
}

#[put("/<id>", format = "json", data = "<message>")]
pub async fn update_message(
    conn: ConnectionPool,
    id: i32,
    message: Json<InputMessage>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Message>, NetworkResponse> {
    let _key = key?;
    let message = logic::message::update_message(&conn, id, message.into_inner()).await;
    Ok(Json(message))
}
