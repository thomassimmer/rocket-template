use crate::db::common::ConnectionPool;
use crate::models::user::{InputUser, User};
use crate::schema::users;
use diesel;
use diesel::prelude::*;

pub async fn get_users(conn: &ConnectionPool) -> Vec<User> {
    conn.run(|c| {
        users::table
            .select(User::as_select())
            .load::<User>(c)
            .expect("Error loading users")
    })
    .await
}

pub async fn get_users_paginated(conn: &ConnectionPool, skip: u32, take: u32) -> Vec<User> {
    conn.run(move |c| {
        users::table
            .select(User::as_select())
            .limit(take.into())
            .offset((skip * take).into())
            .load::<User>(c)
            .expect("Error loading users")
    })
    .await
}

pub async fn get_user(conn: &ConnectionPool, user_id: i32) -> User {
    conn.run(move |c| {
        users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .first::<User>(c)
            .expect("Error loading User")
    })
    .await
}

pub async fn create_user(conn: &ConnectionPool, user: User) -> User {
    conn.run(|c| {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(c)
            .expect("Error saving new user")
    })
    .await
}

pub async fn update_user(conn: &ConnectionPool, user_id: i32, user: InputUser) -> User {
    conn.run(move |c| {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(&user)
            .returning(User::as_returning())
            .get_result(c)
            .unwrap_or_else(|_| panic!("Unable to update user {:?}", user_id))
    })
    .await
}

pub async fn delete_user(conn: &ConnectionPool, user_id: i32) -> bool {
    let num_deleted = conn
        .run(move |c| {
            diesel::delete(users::table.filter(users::id.eq(user_id)))
                .execute(c)
                .expect("Error deleting user")
        })
        .await;

    matches!(num_deleted, 1)
}
