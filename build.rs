const COMMANDS: &[&str] = &["open_sse", "add_on_message_sse", "add_on_error_sse", "add_event_listener_sse", "remove_event_listener_sse", "close_sse"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}

