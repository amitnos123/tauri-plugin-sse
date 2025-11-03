
Use tauri event system `app.emit` to push from rust to frontend
```rust
app.emit("tauri-plugin-sse-URL", ...);
```
where URL is the url

Use sse_client crate
it has
```rust
// Normal message events
    event_source.on_message(|msg| {
        println!("Message: {:?}", msg);
    });

    // Both server-sent and connection-level errors
    event_source.add_event_listener("error", |err| {
        eprintln!("Error or connection problem: {:?}", err);
    });
```

To regocnize it's connection-level errors
```rust
event_source.add_event_listener("error", |err| {
    match err {
        sse_client::EventSourceError::Http(e) => {
            println!("Network/HTTP error: {:?}", e);
        },
        sse_client::EventSourceError::Parse(e) => {
            println!("Parse error: {:?}", e);
        }
    }
});
```

To regocnize it's server sent errors
```rust
event_source.add_event_listener("error", |err| {
    match err {
        sse_client::EventSourceError::Event(msg) => {
            println!("Application-level error from server: {:?}", msg.data);
        }
    }
});
```
