use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenResponse {
  pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseResponse {
  pub value: Option<String>,
}


/* TODO make models
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
*/
