mod alias_merger;
mod alias_parser;
mod alias_runtime;
mod alias_writer;
mod backup;
mod commands;
mod config_paths;
mod error;
mod shell_detect;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::detect_shells,
            commands::get_aliases,
            commands::get_runtime_aliases,
            commands::add_alias,
            commands::update_alias,
            commands::delete_alias,
            commands::delete_external_alias,
            commands::import_alias,
            commands::list_backups,
            commands::restore_backup,
            commands::get_config_paths,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
