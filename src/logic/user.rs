use chrono::Utc;

use crate::db;
use crate::db::common::ConnectionPool;
use crate::models::user::{InputUserCreation, InputUserUpdate, User};

pub async fn get_user(conn: &ConnectionPool, id: i32) -> User {
    db::user::get_user(conn, id).await
}

pub async fn get_users(conn: &ConnectionPool) -> Vec<User> {
    db::user::get_users(conn).await
}

pub async fn get_users_paginated(conn: &ConnectionPool, skip: u32, take: u32) -> Vec<User> {
    db::user::get_users_paginated(conn, skip, take).await
}

pub async fn delete_user<'a>(conn: &ConnectionPool, id: i32) -> bool {
    db::user::delete_user(conn, id).await
}

pub async fn create_user<'a>(conn: &ConnectionPool, new_user: InputUserCreation) -> User {
    let user = User {
        id: None,
        user_name: new_user.user_name,
        password: new_user.password, // todo: hash
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        creation_date: Utc::now().naive_utc(),
    };

    db::user::create_user(conn, user).await
}

pub async fn update_user<'a>(conn: &ConnectionPool, id: i32, user: InputUserUpdate) -> User {
    db::user::update_user(conn, id, user).await
}
