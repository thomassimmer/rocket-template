use std::sync::Arc;

use crate::{db::common::ConnectionPool, models::message::Message};

use super::app_state::AppState;

pub struct Task {
    pub id: i32,
    pub message: Message,
    pub app_state: AppState,
    pub conn: Arc<ConnectionPool>,
}
