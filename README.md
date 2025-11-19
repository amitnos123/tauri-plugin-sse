# Tauri Plugin SSE

![Crates.io Version](https://img.shields.io/crates/v/tauri-plugin-sse)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg)

[Server-Sent Events (SSE)](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events) is a web technology that allows a server to push real-time updates to a client over a long-lived HTTP connection.

It provides a lightweight, one-way communication channel, making it ideal for applications that need continuous data streaming from the backend without using [WebSockets](https://tauri.app/plugin/websocket/).

SSE is a simple solution for notifications, activity feeds, live status updates, or any scenario where the server needs to send real-time updates to the client.

This project uses the [`sse_client`](https://crates.io/crates/sse-client) crate for handling SSE connections on the Rust side (Tauri's backend).

**Sample Usage:**

* Realtime activity feeds or notifications
* Live server logs
* Monitoring application state
* Synchronizing lightweight UI updates

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | not tested         |
| Android  | not tested         |
| iOS      | not tested         |

## Installation

- Crate: https://crates.io/crates/tauri-plugin-sse
  - `cargo add tauri-plugin-sse`
<!---
- NPM Package: https://www.npmjs.com/package/tauri-plugin-sse-api
  - `npm install tauri-plugin-sse-api`
  
- YARN Package: 
  - `yarn install tauri-plugin-sse-api`

- PNPM Package: 
  - `pnpm install tauri-plugin-sse-api`

- DENO Package: 
  - `deno install tauri-plugin-sse-api`

- BUN Package: 
  - `bun install tauri-plugin-sse-api`
--->

## Usage

### TypeScript/JavaScript

```js
import EventSource from "tauri-plugin-sse";

// Create connection to server endpoint
const source = new EventSource("https://example.com/events");

// Handle on establishing connection
source.onopen = () => {
  console.log("Connection stabilished!");
};

// Fired when a message is received
source.onmessage = (event) => {
  console.log("Message from server:", event.data);
};

// Handle errors
source.onerror = (err) => {
  console.error("EventSource failed:", err);
};

// Handle named event types
source.addEventListener("ping", (event) => {
  console.log("Ping:", event.data);
});

// Remove handler
removeEventListener("ping");

// Close Event Source
source.close();
```

### Rust

```rust
//  TODO: Add use of right modules

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
```

## Licenses

MIT or MIT/Apache 2.0 where applicable.
