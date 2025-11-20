use tauri::{
    Emitter,
    Manager,
    AppHandle,
    command,
    Runtime,
    async_runtime::Mutex,
    ipc::{
        CommandScope,
        GlobalScope
    }
};

use std::collections::HashMap;

extern crate sse_client;
use sse_client::EventSource;

use crate::models::*;
use crate::Result;

use crate::scope::{Entry, Scope};

pub(crate) fn event_full_name(url: &str, name: &str) -> String {
    let event_start_name = "tauri-plugin-sse-";
    event_start_name.to_string() + &sanitize_event_name(url) + "-" + name
}

/// Make a string safe for Tauri event names
fn sanitize_event_name(url: &str) -> String {
    url.replace("://", "__")
       .chars()
       .map(|c| match c {
           'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '/' | ':' | '_' => c,
           _ => '_',
       })
       .collect()
}


#[derive(Default)]
pub struct AppState {
  pub events: HashMap<String, EventSource>
}

/*
    command_scope: CommandScope<Entry>,
    global_scope: GlobalScope<Entry>
*/

#[command]
pub(crate) async fn open_sse<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    command_scope: CommandScope<Entry>,
    global_scope: GlobalScope<Entry>
) -> Result<bool> {
    if Scope::new(
                command_scope
                    .allows()
                    .iter()
                    .chain(global_scope.allows())
                    .collect(),
                command_scope
                    .denies()
                    .iter()
                    .chain(global_scope.denies())
                    .collect(),
            )
            .is_allowed(&url::Url::parse(&url).unwrap())
            {
                let state: tauri::State<'_, Mutex<AppState>> = app.state::<Mutex<AppState>>();

                // Create a new EventSource instance for the given URL.
                let event_source = EventSource::new(&url).unwrap();

                println!("Opening SSE connection to {url}");

                // Lock the mutex to mutably access the state.
                let mut state = state.lock().await;
                state.events.insert(url, event_source);

                return Ok(true);
            }
    return Ok(false);
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
        println!("{:?}", event_source.state());
        event_source.on_message(move |message| {
                    let payload = TauriEventSse::from_client_event_sse(message);
                    println!("add_on_message_sse payload: {:?}", payload);
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
        event_source.add_event_listener(&name ,|_event| {}); // Do nothing on event
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
