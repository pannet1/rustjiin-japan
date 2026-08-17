#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod features;

use std::thread;
use tokio::runtime::Runtime;

fn main() {
    // Fix Linux transparent window compositing bug (fixes DRM permission denied errors)
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    
    // Start Axum server in a separate background thread
    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            features::core::server::start_axum_server().await;
        });
    });

    tauri::Builder::default()
        .setup(|_app| {
            // Give Axum a tiny moment to bind to the port before the webview loads
            std::thread::sleep(std::time::Duration::from_millis(500));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
