use crate::db::common::ConnectionPool;
use crate::models::user::{InputUserUpdate, User};
use crate::schema::users;
use diesel;
use diesel::prelude::*;

pub async fn get_users(conn: &ConnectionPool, skip: Option<u32>, take: Option<u32>) -> Vec<User> {
    let mut query = users::table.select(User::as_select()).into_boxed();

    if let Some(take) = take {
        query = query.limit(take as i64);
    }

    if let Some(skip) = skip {
        query = query.offset(skip as i64)
    }

    conn.run(move |c| query.load::<User>(c).expect("Error loading users"))
        .await
}

pub async fn get_user(conn: &ConnectionPool, user_id: i32) -> Result<User, diesel::result::Error> {
    conn.run(move |c| {
        users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .first::<User>(c)
    })
    .await
}

pub async fn get_user_by_user_name_and_password(
    conn: &ConnectionPool,
    user_name: String,
    password: String,
) -> Result<User, diesel::result::Error> {
    conn.run(move |c| {
        users::table
            .filter(users::user_name.eq(user_name))
            .filter(users::password.eq(password)) // todo: hash
            .select(User::as_select())
            .first::<User>(c)
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

pub async fn update_user(conn: &ConnectionPool, user_id: i32, user: InputUserUpdate) -> User {
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
