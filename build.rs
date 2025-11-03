use std::env;
use std::path::Path;

const COMMANDS: &[&str] = &["ping"];

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let permissions_dir = Path::new(&out_dir).join("permissions");

    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .out_dir(permissions_dir) // <-- write generated files here
        .build();
}

