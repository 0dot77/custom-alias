use crate::config_paths;
use crate::types::{DetectedShell, ShellType};
use std::path::Path;

pub fn detect_shells() -> Vec<DetectedShell> {
    let default_shell = std::env::var("SHELL").unwrap_or_default();
    let mut shells = vec![];

    let candidates: Vec<(ShellType, Vec<&str>)> = vec![
        (
            ShellType::Bash,
            if cfg!(target_os = "windows") {
                vec![
                    "C:\\Program Files\\Git\\bin\\bash.exe",
                    "C:\\Program Files (x86)\\Git\\bin\\bash.exe",
                ]
            } else {
                vec!["/bin/bash", "/usr/local/bin/bash", "/opt/homebrew/bin/bash"]
            },
        ),
        (
            ShellType::Zsh,
            if cfg!(target_os = "windows") {
                vec![]
            } else {
                vec!["/bin/zsh", "/usr/local/bin/zsh", "/opt/homebrew/bin/zsh"]
            },
        ),
        (
            ShellType::Fish,
            if cfg!(target_os = "windows") {
                vec!["C:\\Program Files\\fish\\bin\\fish.exe"]
            } else {
                vec![
                    "/usr/local/bin/fish",
                    "/usr/bin/fish",
                    "/opt/homebrew/bin/fish",
                ]
            },
        ),
        (
            ShellType::PowerShell,
            if cfg!(target_os = "windows") {
                vec![
                    "C:\\Program Files\\PowerShell\\7\\pwsh.exe",
                    "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
                ]
            } else {
                vec![
                    "/usr/local/bin/pwsh",
                    "/opt/homebrew/bin/pwsh",
                    "/usr/bin/pwsh",
                ]
            },
        ),
    ];

    for (shell_type, paths) in candidates {
        // Try known paths first, then fall back to `which`
        let binary_path = paths
            .iter()
            .find(|p| Path::new(p).exists())
            .map(|p| p.to_string())
            .or_else(|| find_in_path(&shell_type));

        if let Some(binary) = binary_path {
            let config_files = config_paths::get_config_files(&shell_type)
                .into_iter()
                .filter(|p| p.exists())
                .map(|p| p.to_string_lossy().to_string())
                .collect();

            let is_default = default_shell.contains(&shell_type.to_string());

            shells.push(DetectedShell {
                shell_type,
                binary_path: binary,
                config_files,
                is_default,
            });
        }
    }

    shells
}

fn find_in_path(shell: &ShellType) -> Option<String> {
    let cmd = shell.to_string();
    let which_cmd = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    std::process::Command::new(which_cmd)
        .arg(&cmd)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().lines().next().unwrap_or("").to_string())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
}
