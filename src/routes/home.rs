use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Request;

use crate::db::common::ConnectionPool;
use crate::logic;
use crate::models::user::InputUserCreation;
use crate::structs::request::LoginRequest;
use crate::structs::response::{NetworkResponse, Response, ResponseBody};
use crate::utils::jwt::create_jwt;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, favicon, register_user, login_user]
}

#[get("/")]
fn index() -> &'static str {
    "Rocket-template is running."
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("src/static/favicon.ico").await.ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[post("/register", format = "application/json", data = "<user>")]
pub async fn register_user(
    conn: ConnectionPool,
    user: Json<InputUserCreation>,
) -> Result<String, NetworkResponse> {
    let user = logic::user::create_user(&conn, user.into_inner()).await;

    let token = match create_jwt(user.id.expect("No user id.")) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }?;

    let response = Response {
        body: ResponseBody::AuthToken(token),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login_user(
    conn: ConnectionPool,
    user: Json<LoginRequest>,
) -> Result<String, NetworkResponse> {
    let token = logic::home::login_user(&conn, user).await?;

    let response = Response {
        body: ResponseBody::AuthToken(token),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
