use rocket::{
    response::stream::{Event, EventStream},
    Shutdown, State,
};
use tokio::{select, sync::broadcast::error::RecvError};

use crate::structs::app_state::AppState;

#[get("/")]
pub async fn events(app_state: &State<AppState>, mut end: Shutdown) -> EventStream![] {
    let mut rx = app_state.message_sender.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg)
        }
    }
}
