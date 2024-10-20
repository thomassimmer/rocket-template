use rocket::{delete, get, post, put, routes, State};

use crate::db::common::ConnectionPool;
use crate::logic;
use crate::models::message::{InputMessage, Message};
use crate::structs::app_state::AppState;
use crate::structs::jwt::JWT;
use crate::structs::response::NetworkResponse;
use crate::utils::worker::revoke_task;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_messages,
        get_message,
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

#[get("/?<skip>&<take>")]
pub async fn get_messages(
    conn: ConnectionPool,
    skip: Option<u32>,
    take: Option<u32>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<Vec<Message>>, NetworkResponse> {
    let _key = key?;
    let messages = logic::message::get_messages(&conn, skip, take).await;

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
    app_state: &State<AppState>,
) -> Result<String, NetworkResponse> {
    let key = key?;
    let user_id = key.claims.subject_id;

    logic::message::create_message(conn, message.into_inner(), user_id, app_state).await;

    Ok("Ok".to_string())
}

#[get("/<id>")]
pub async fn stop_message_creation(
    id: i32,
    app_state: &State<AppState>,
) -> Result<String, NetworkResponse> {
    revoke_task(id, app_state.worker_queues.get("gpt-4o").unwrap()).await;

    Ok("Ok".to_string())
}

#[put("/<id>", format = "json", data = "<message>")]
pub async fn update_message(
    conn: ConnectionPool,
    id: i32,
    message: Json<InputMessage>,
    key: Result<JWT, NetworkResponse>,
    app_state: &State<AppState>,
) -> Result<String, NetworkResponse> {
    let _key = key?;

    revoke_task(id, app_state.worker_queues.get("gpt-4o").unwrap()).await;

    logic::message::update_message(conn, id, message.into_inner(), app_state).await;

    Ok("Ok".to_string())
}
