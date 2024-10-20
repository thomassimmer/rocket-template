#[macro_use]
extern crate rocket;

pub mod models {
    pub mod message;
    pub mod user;
}

pub mod routes {
    pub mod home;
    pub mod message;
    pub mod message_channel;
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
    pub mod worker;
}

pub mod structs {
    pub mod app_state;
    pub mod jwt;
    pub mod request;
    pub mod response;
    pub mod task;
    pub mod worker_queue;
}

use std::{collections::HashMap, sync::Arc};

use crate::utils::common::cors_fairing;
use db::common::{run_migrations, ConnectionPool};
use models::message::Message;
use rocket::{
    fairing::AdHoc,
    fs::{relative, FileServer},
    tokio::{
        spawn,
        sync::{broadcast::channel, Notify},
    },
};
use routes::message_channel::events;
use structs::{app_state::AppState, worker_queue::WorkerQueue};
use tokio::sync::Mutex;
use utils::worker::worker;

#[launch]
pub fn rocket() -> _ {
    let worker_queues: HashMap<String, WorkerQueue> = HashMap::from([
        (
            "gpt-4-32k".to_string(),
            WorkerQueue {
                name: "gpt-4-32k".to_string(),
                task_queue: Arc::new(Mutex::new(Vec::new())),
                notifier: Arc::new(Notify::new()),
                max_parallel_task: 2,
            },
        ),
        (
            "gpt-4o".to_string(),
            WorkerQueue {
                name: "gpt-4o".to_string(),
                task_queue: Arc::new(Mutex::new(Vec::new())),
                notifier: Arc::new(Notify::new()),
                max_parallel_task: 2,
            },
        ),
    ]);

    let message_sender = Arc::new(channel::<Message>(1024).0);
    let app_state = AppState {
        worker_queues: worker_queues.clone(),
        message_sender,
    };

    for queue in worker_queues.values() {
        spawn(worker(queue.clone()));
    }

    rocket::build()
        .manage(app_state)
        .mount("/", routes::home::get_routes())
        .mount("/users", routes::user::get_routes())
        .mount("/messages", routes::message::get_routes())
        .mount("/static", FileServer::from(relative!("src/static")))
        .mount("/ws/", routes![events])
        .register("/", catchers![routes::home::not_found])
        .attach(cors_fairing())
        .attach(ConnectionPool::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
}
