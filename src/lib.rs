#[macro_use]
extern crate rocket;

pub mod models {
    pub mod message;
    pub mod user;
}

pub mod routes {
    pub mod home;
    pub mod message;
    pub mod user;
}

pub mod schema;

pub mod logic {
    pub mod home;
    pub mod message;
    pub mod user;
}

pub mod db {
    pub mod common;
    pub mod message;
    pub mod user;
}

pub mod utils {
    pub mod common;
    pub mod jwt;
}

pub mod structs {
    pub mod jwt;
    pub mod request;
    pub mod response;
}

use crate::utils::common::cors_fairing;
use db::common::{run_migrations, ConnectionPool};
use rocket::{
    fairing::AdHoc,
    fs::{relative, FileServer},
};

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::home::get_routes())
        .mount("/users", routes::user::get_routes())
        .mount("/messages", routes::message::get_routes())
        .mount("/static", FileServer::from(relative!("src/static")))
        .register("/", catchers![routes::home::not_found])
        .attach(cors_fairing())
        .attach(ConnectionPool::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
}
