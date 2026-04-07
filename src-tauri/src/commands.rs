use crate::alias_merger;
use crate::alias_parser;
use crate::alias_runtime;
use crate::alias_writer;
use crate::backup;
use crate::config_paths;
use crate::error::AppError;
use crate::shell_detect;
use crate::types::*;
use std::path::Path;

#[tauri::command]
pub fn detect_shells() -> Vec<DetectedShell> {
    shell_detect::detect_shells()
}

#[tauri::command]
pub fn get_aliases(shell: ShellType) -> Result<Vec<MergedAlias>, AppError> {
    let shells = shell_detect::detect_shells();
    let detected = shells.iter().find(|s| s.shell_type == shell);

    let config_files = config_paths::get_config_files(&shell);
    let mut file_aliases = vec![];
    for path in &config_files {
        if path.exists() {
            file_aliases.extend(alias_parser::parse_config_file(path, &shell));
        }
    }

    let runtime_aliases = if let Some(s) = detected {
        alias_runtime::query_runtime_aliases(&shell, &s.binary_path)
    } else {
        vec![]
    };

    Ok(alias_merger::merge_aliases(file_aliases, runtime_aliases))
}

#[tauri::command]
pub fn get_runtime_aliases(shell: ShellType) -> Result<Vec<RuntimeAlias>, AppError> {
    let shells = shell_detect::detect_shells();
    let detected = shells.iter().find(|s| s.shell_type == shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: shell.to_string(),
        }
    })?;

    Ok(alias_runtime::query_runtime_aliases(
        &shell,
        &detected.binary_path,
    ))
}

#[tauri::command]
pub fn add_alias(input: AliasInput) -> Result<Alias, AppError> {
    alias_writer::add_alias(&input)
}

#[tauri::command]
pub fn update_alias(old_name: String, input: AliasInput) -> Result<Alias, AppError> {
    alias_writer::update_alias(&old_name, &input)
}

#[tauri::command]
pub fn delete_alias(name: String, shell: ShellType) -> Result<(), AppError> {
    alias_writer::delete_alias(&name, &shell)
}

#[tauri::command]
pub fn import_alias(name: String, shell: ShellType) -> Result<Alias, AppError> {
    alias_writer::import_alias(&name, &shell)
}

#[tauri::command]
pub fn list_backups(shell: ShellType) -> Result<Vec<BackupInfo>, AppError> {
    backup::list_backups(&shell)
}

#[tauri::command]
pub fn restore_backup(backup_path: String) -> Result<(), AppError> {
    backup::restore_backup(Path::new(&backup_path))
}

#[tauri::command]
pub fn get_config_paths(shell: ShellType) -> Vec<String> {
    config_paths::get_config_files(&shell)
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
