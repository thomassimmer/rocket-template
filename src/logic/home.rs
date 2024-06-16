use rocket::serde::json::Json;

use crate::db::common::ConnectionPool;
use crate::db::user::get_user_by_user_name_and_password;
use crate::models::user::User;
use crate::structs::request::LoginRequest;
use crate::structs::response::{NetworkResponse, Response, ResponseBody};
use crate::utils::jwt::create_jwt;

pub async fn login_user(
    conn: &ConnectionPool,
    user: Json<LoginRequest>,
) -> Result<String, NetworkResponse> {
    let user = user.into_inner();

    let found_user: User =
        match get_user_by_user_name_and_password(conn, user.user_name.clone(), user.password).await
        {
            Ok(user) => user,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error - Wrong username or password for user {}",
                            &user.user_name
                        )),
                    };
                    return Err(NetworkResponse::NotFound(
                        serde_json::to_string(&response).unwrap(),
                    ));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    match create_jwt(found_user.id.expect("No user id.")) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}
