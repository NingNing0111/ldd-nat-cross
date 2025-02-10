use sys::init_app_dir;

pub mod client;
pub mod command;
pub mod common;
pub mod core;
pub mod global;
pub mod handler;
pub mod helper;
pub mod model;
pub mod store;
pub mod sys;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            init_app_dir();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::client::start_app,
            command::client::stop_app,
            command::client::last_config,
            command::home::count_info,
            command::log::add_connect_log,
            command::log::page_connect_log,
            command::config::add_server_config,
            command::config::page_server_config,
            command::config::add_proxy_config_batch,
            command::config::page_proxy_config,
            command::config::del_server_config,
            command::config::del_proxy_config,
            command::config::ping,
            command::config::update_server_config,
            command::config::update_proxy_config,
            command::setting::get_setting,
            command::setting::update_setting,
            command::setting::reset_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
