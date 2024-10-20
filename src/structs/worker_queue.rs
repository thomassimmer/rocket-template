use std::sync::Arc;

use tokio::sync::{Mutex, Notify};

use super::task::Task;

#[derive(Clone)]
pub struct WorkerQueue {
    pub name: String,
    pub task_queue: Arc<Mutex<Vec<Task>>>,
    pub notifier: Arc<Notify>,
    pub max_parallel_task: u8,
}
