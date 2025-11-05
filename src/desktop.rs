use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use sse_client::Event;

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Sse<R>> {
  Ok(Sse(app.clone()))
}

/// Access to the sse APIs.
pub struct Sse<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Sse<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
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
event_source.close();
*/

  pub fn on_open(listener: F) where F: Fn() + Send + 'static {
    
  }

  pub fn on_message(listener: F) where F: Fn(Event) + Send + 'static {
    
  }

  pub fn on_error(listener: F) where F: Fn(Event) + Send + 'static {
    
  }

  pub fn add_event_listener(name: &str, listener: F) where F: Fn(Event) + Send + 'static {
    
  }

  pub fn remove_event_listener(name: &str) -> () {
    
  }

  pub fn close() -> () {
    
  }
}
