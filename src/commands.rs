use tauri::{AppHandle, command, Runtime};

use std::collections::HashMap;

use crate::models::*;
use crate::Result;
use crate::SseExt;

#[derive(Default)]
struct AppState {
  events: HashMap<String, EventSource>
}

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.sse().ping(payload)
}

/*
    command_scope: CommandScope<Entry>,
    global_scope: GlobalScope<Entry>
*/

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    let state = app.state::<Mutex<AppState>>();

    let url = "http://event-stream-address/sub".to_string();
    let event_source = EventSource::new("http://event-stream-address/sub").unwrap();

    // Lock the mutex to mutably access the state.
    let mut state = state.lock().unwrap();
    state.events.insert(url, event_source);
}

/*TOOD implemented
  // Create connection to server endpoint
let event_source = EventSource::new("http://event-stream-address/sub").unwrap();

// Handle on establishing connection
event_source.on_open(|| {
    println!("Connection stabilished!");
});

// Fired when a message is received
event_source.on_message(|message| {
    println!("New message event {:?}", message);
});

// Handle errors
event_source.on_error(|error| {
    println!("Error {:?}", error);
});

// Handle named event types
event_source.add_event_listener("myEvent", |event| {
    println!("Event {} received: {}", event.type_, event.data);
});

// Remove handler
event_source.remove_event_listener("myEvent");

// Close Event Source
event_source.close
