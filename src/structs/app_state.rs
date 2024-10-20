use std::{collections::HashMap, sync::Arc};

use rocket::tokio::sync::broadcast::Sender;

use crate::models::message::Message;

use super::worker_queue::WorkerQueue;

#[derive(Clone)]
pub struct AppState {
    pub worker_queues: HashMap<String, WorkerQueue>,
    pub message_sender: Arc<Sender<Message>>,
}
