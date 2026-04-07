use crate::config_paths;
use crate::error::AppError;
use crate::types::{BackupInfo, ShellType};
use chrono::Local;
use std::path::Path;

pub fn create_backup(config_path: &Path, shell: &ShellType) -> Result<BackupInfo, AppError> {
    let backup_dir = config_paths::get_backup_dir().ok_or_else(|| AppError::Io(
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot determine backup directory"),
    ))?;

    std::fs::create_dir_all(&backup_dir)?;

    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let file_name = config_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let backup_name = format!("{}_{}.bak", file_name, timestamp);
    let backup_path = backup_dir.join(&backup_name);

    std::fs::copy(config_path, &backup_path)?;

    // Prune old backups, keep last 10 per shell
    prune_backups(&backup_dir, &file_name, 10)?;

    Ok(BackupInfo {
        path: backup_path.to_string_lossy().to_string(),
        shell: shell.clone(),
        created_at: now.to_rfc3339(),
        original_file: config_path.to_string_lossy().to_string(),
    })
}

fn prune_backups(
    backup_dir: &Path,
    file_prefix: &str,
    keep: usize,
) -> Result<(), AppError> {
    let mut backups: Vec<_> = std::fs::read_dir(backup_dir)?
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with(file_prefix)
        })
        .collect();

    backups.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH));
    backups.reverse();

    for old in backups.into_iter().skip(keep) {
        let _ = std::fs::remove_file(old.path());
    }

    Ok(())
}

pub fn list_backups(shell: &ShellType) -> Result<Vec<BackupInfo>, AppError> {
    let backup_dir = match config_paths::get_backup_dir() {
        Some(d) if d.exists() => d,
        _ => return Ok(vec![]),
    };

    let config_files = config_paths::get_config_files(shell);
    let prefixes: Vec<String> = config_files
        .iter()
        .filter_map(|p| p.file_name())
        .map(|f| f.to_string_lossy().to_string())
        .collect();

    let mut backups = vec![];
    for entry in std::fs::read_dir(&backup_dir)?.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".bak") {
            continue;
        }
        let matches_prefix = prefixes.iter().any(|p| name.starts_with(p.as_str()));
        if !matches_prefix {
            continue;
        }

        let created_at = entry
            .metadata()
            .and_then(|m| m.modified())
            .map(|t| {
                let dt: chrono::DateTime<Local> = t.into();
                dt.to_rfc3339()
            })
            .unwrap_or_default();

        let original_file = prefixes
            .iter()
            .find(|p| name.starts_with(p.as_str()))
            .cloned()
            .unwrap_or_default();

        backups.push(BackupInfo {
            path: entry.path().to_string_lossy().to_string(),
            shell: shell.clone(),
            created_at,
            original_file,
        });
    }

    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(backups)
}

pub fn restore_backup(backup_path: &Path) -> Result<(), AppError> {
    // Extract original file name from backup name (e.g., ".zshrc_20240101_120000.bak" -> ".zshrc")
    let backup_name = backup_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Find the original config file path
    // Backup name format: {original_name}_{timestamp}.bak
    let original_name = backup_name
        .rsplit_once('_')
        .and_then(|(rest, _)| rest.rsplit_once('_'))
        .map(|(name, _)| name)
        .unwrap_or(&backup_name);

    let home = dirs::home_dir().ok_or_else(|| {
        AppError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine home directory",
        ))
    })?;

    let target = home.join(original_name);
    if !target.parent().is_some_and(|p| p.exists()) {
        return Err(AppError::ConfigNotFound {
            path: target.to_string_lossy().to_string(),
        });
    }

    std::fs::copy(backup_path, &target)?;
    Ok(())
}
