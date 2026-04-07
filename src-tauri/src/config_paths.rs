use crate::types::ShellType;
use std::path::PathBuf;

pub fn get_config_files(shell: &ShellType) -> Vec<PathBuf> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return vec![],
    };

    match shell {
        ShellType::Bash => {
            let mut paths = vec![
                home.join(".bashrc"),
                home.join(".bash_profile"),
                home.join(".bash_aliases"),
            ];
            #[cfg(target_os = "macos")]
            paths.push(home.join(".profile"));
            paths
        }
        ShellType::Zsh => {
            vec![home.join(".zshrc"), home.join(".zshenv")]
        }
        ShellType::Fish => {
            let config_dir = dirs::config_dir().unwrap_or_else(|| home.join(".config"));
            let fish_dir = config_dir.join("fish");
            let mut paths = vec![fish_dir.join("config.fish")];
            // Also check conf.d directory
            let conf_d = fish_dir.join("conf.d");
            if conf_d.exists() {
                if let Ok(entries) = std::fs::read_dir(&conf_d) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().is_some_and(|ext| ext == "fish") {
                            paths.push(path);
                        }
                    }
                }
            }
            paths
        }
        ShellType::PowerShell => {
            let mut paths = vec![];
            #[cfg(target_os = "windows")]
            {
                // PowerShell 7+
                if let Some(docs) = dirs::document_dir() {
                    paths.push(docs.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"));
                    // PowerShell 5.1
                    paths.push(
                        docs.join("WindowsPowerShell")
                            .join("Microsoft.PowerShell_profile.ps1"),
                    );
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                let config_dir = dirs::config_dir().unwrap_or_else(|| home.join(".config"));
                paths.push(config_dir.join("powershell").join("profile.ps1"));
            }
            paths
        }
    }
}

/// Returns the target file where new aliases should be written
pub fn get_write_target(shell: &ShellType) -> Option<PathBuf> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return None,
    };

    match shell {
        ShellType::Bash => Some(home.join(".bashrc")),
        ShellType::Zsh => Some(home.join(".zshrc")),
        ShellType::Fish => {
            let config_dir = dirs::config_dir().unwrap_or_else(|| home.join(".config"));
            Some(config_dir.join("fish").join("conf.d").join("custom-alias.fish"))
        }
        ShellType::PowerShell => {
            #[cfg(target_os = "windows")]
            {
                dirs::document_dir()
                    .map(|docs| docs.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"))
            }
            #[cfg(not(target_os = "windows"))]
            {
                let config_dir = dirs::config_dir().unwrap_or_else(|| home.join(".config"));
                Some(config_dir.join("powershell").join("profile.ps1"))
            }
        }
    }
}

pub fn get_backup_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|c| c.join("custom-alias").join("backups"))
}
