use tauri::{Emitter, Manager, State, AppHandle, command, Runtime, async_runtime::Mutex};

use std::collections::HashMap;

use sse_client::EventSource;

use crate::models::*;
use crate::Result;
use crate::SseExt;

pub(crate) fn event_full_name(url: &str, name: &str) -> String {
    let event_start_name = "tauri-plugin-sse-";
    event_start_name.to_string() + url + "-" + name
}

#[derive(Default)]
pub struct AppState {
  pub events: HashMap<String, EventSource>
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
    url: String,
) -> Result<bool> {
    let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();

    // Create a new EventSource instance for the given URL.
    let event_source = EventSource::new(&url).unwrap();

    // Lock the mutex to mutably access the state.
    let mut state = state.lock().await;
    state.events.insert(url, event_source);

    Ok(true)
}

#[command]
pub(crate) async fn add_on_message_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
) -> Result<bool> {
    let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();
    let mut state = state.lock().await;
    
    let app_clone = app.clone();
    let event_full_name = event_full_name(&url, "on_message");
    if let Some(event_source) = state.events.get_mut(&url) {
        event_source.on_message(move |message| {
                    let payload = TauriEventSse::from_client_event_sse(message);
                    app_clone.emit(&event_full_name, payload).unwrap();
                }
            );
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub(crate) async fn add_on_error_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
) -> Result<bool> {
    let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();
    let mut state = state.lock().await;

    if let Some(event_source) = state.events.get_mut(&url) {
        let app_clone = app.clone();
        let event_full_name = event_full_name(&url, "error");
        event_source.add_event_listener("error" ,move |event| {
                    let payload = TauriEventSse::from_client_event_sse(event);
                    app_clone.emit(&event_full_name, payload).unwrap();
                }
            );
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub(crate) async fn add_event_listener_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    name: String
) -> Result<bool> {
    let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();
    let mut state = state.lock().await;

    if let Some(event_source) = state.events.get_mut(&url) {
        let app_clone = app.clone();
        let name_clone = name.clone();
        let event_full_name = event_full_name(&url, &name_clone);
        event_source.add_event_listener(&name ,move |event| {
                    let payload = TauriEventSse::from_client_event_sse(event);
                    app_clone.emit(&event_full_name, payload).unwrap();
                }
            );
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub(crate) async fn remove_event_listener_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    name: String
) -> Result<bool> {
    let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();
    let mut state = state.lock().await;

    if let Some(event_source) = state.events.get_mut(&url) {
        event_source.add_event_listener(&name ,|event| {}); // Do nothing on event
        return Ok(true);
    }
    Ok(false)
}

#[command]
pub(crate) async fn close_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
) -> Result<bool> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().await;

    // Remove the event source from the map
    if let Some(event_source) = state.events.remove(&url) {
        // Now that it's removed, no borrows exist → safe to close
        event_source.close();

        return Ok(true);
    }
    Ok(false)
}
