use rocket::{delete, get, post, put, routes};

use crate::db::common::ConnectionPool;
use crate::logic;
use crate::models::user::{InputUser, User};
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_users,
        get_user,
        get_users_paginated,
        delete_user,
        create_user,
        update_user
    ]
}

#[get("/<id>")]
pub async fn get_user(conn: ConnectionPool, id: i32) -> Json<User> {
    let user = logic::user::get_user(&conn, id).await;
    Json(user)
}

#[get("/")]
pub async fn get_users(conn: ConnectionPool) -> Json<Vec<User>> {
    let userss = logic::user::get_users(&conn).await;
    Json(userss)
}

#[get("/?<skip>&<take>")]
pub async fn get_users_paginated(conn: ConnectionPool, skip: u32, take: u32) -> Json<Vec<User>> {
    let userss = logic::user::get_users_paginated(&conn, skip, take).await;
    Json(userss)
}

#[delete("/<id>")]
pub async fn delete_user(conn: ConnectionPool, id: i32) -> String {
    logic::user::delete_user(&conn, id).await;
    "Ok".to_string()
}

#[post("/", format = "json", data = "<user>")]
pub async fn create_user(conn: ConnectionPool, user: Json<InputUser>) -> Json<User> {
    let user = logic::user::create_user(&conn, user.into_inner()).await;
    Json(user)
}

#[put("/<id>", format = "json", data = "<user>")]
pub async fn update_user(conn: ConnectionPool, id: i32, user: Json<InputUser>) -> Json<User> {
    let user = logic::user::update_user(&conn, id, user.into_inner()).await;
    Json(user)
}
