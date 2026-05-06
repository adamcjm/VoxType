pub mod audio;
pub mod commands;
pub mod config;
pub mod error;
pub mod history;
pub mod hotkey;
pub mod llm;
pub mod model_manager;
pub mod output;
pub mod paths;
pub mod pipeline;
pub mod state;
pub mod stt;

#[cfg(target_os = "macos")]
pub mod hotkey_macos;

use state::AppState;
use std::sync::Arc;
use tauri::{Manager, Emitter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn run() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("VoxType starting up");

    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(app_state.clone())
        .setup(move |app| {
            tracing::info!("Tauri app setup complete");

            // Check if onboarding is needed
            let needs_onboarding = {
                let state = app.state::<Arc<AppState>>();
                let config = state.config.lock().unwrap();
                config.needs_onboarding()
            };

            if needs_onboarding {
                tracing::info!("No API keys — showing Settings panel");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            // Register Fn key hotkey on macOS
            #[cfg(target_os = "macos")]
            hotkey_macos::register_fn_hotkey();

            // Background task: poll Fn key toggle and emit to frontend
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                    #[cfg(target_os = "macos")]
                    {
                        if hotkey_macos::poll_toggle() {
                            // Emit event to frontend
                            let _ = app_handle.emit("hotkey:toggle", ());
                        }
                    }

                    #[cfg(not(target_os = "macos"))]
                    {
                        // On non-macOS, poll rdev or other hotkey mechanism
                    }
                }
            });

            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::recording::start_recording,
            commands::recording::stop_recording,
            commands::recording::toggle_recording,
            commands::recording::list_audio_devices,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::needs_onboarding,
            commands::history::get_history,
            commands::history::add_history_item,
            commands::history::remove_history_item,
            commands::clipboard::copy_to_clipboard,
            commands::clipboard::paste_text,
            commands::model::list_models,
            commands::model::download_model,
            commands::model::delete_model,
            commands::model::model_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running VoxType");
}
