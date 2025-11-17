use tauri::{AppHandle, command, Runtime, async_runtime::Mutex};

use std::collections::HashMap;

use sse_client::EventSource;

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
pub(crate) async fn open_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: OpenRequest,
) -> Result<OpenResponse> {
    let state = app.state::<Mutex<AppState>>();

    let url = "http://event-stream-address/sub".to_string();
    let event_source = EventSource::new("http://event-stream-address/sub").unwrap();

    // Lock the mutex to mutably access the state.
    let mut state = state.lock().unwrap();
    state.events.insert(url, event_source);
}

#[command]
pub(crate) async fn on_message_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: OnMessageRequest,
) -> Result<OnMessageResponse> {
}

#[command]
pub(crate) async fn on_error_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: OnErrorRequest,
) -> Result<OnErrorResponse> {
}

#[command]
pub(crate) async fn add_event_listener_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: AddEventListenerRequest,
) -> Result<AddEventListenerResponse> {
}

#[command]
pub(crate) async fn remove_event_listener_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: RemoveEventListenerRequest,
) -> Result<RemoveEventListenerResponse> {
}

#[command]
pub(crate) async fn close_sse<R: Runtime>(
    app: AppHandle<R>,
    payload: CloseRequest,
) -> Result<CloseResponse> {
    let url = "http://event-stream-address/sub".to_string();
    let state = app.state::<Mutex<AppState>>();
    if state.events.contains_key(url) {
        let event_source = state.get(url);
        event_source.close();
        let mut state = state.lock().unwrap();
        state.events.remove(url)
    }
}
