use crate::types::{RuntimeAlias, ShellType};
use std::time::Duration;

pub fn query_runtime_aliases(shell: &ShellType, binary_path: &str) -> Vec<RuntimeAlias> {
    let (program, args) = match shell {
        ShellType::Bash => (binary_path.to_string(), vec!["-ic".to_string(), "alias".to_string()]),
        ShellType::Zsh => (binary_path.to_string(), vec!["-ic".to_string(), "alias".to_string()]),
        ShellType::Fish => (binary_path.to_string(), vec!["-c".to_string(), "alias".to_string()]),
        ShellType::PowerShell => (
            binary_path.to_string(),
            vec![
                "-NoProfile".to_string(),
                "-Command".to_string(),
                "Get-Alias | ForEach-Object { \"$($_.Name)=$($_.Definition)\" }".to_string(),
            ],
        ),
    };

    let output = match std::process::Command::new(&program)
        .args(&args)
        .env("TERM", "dumb")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(child) => {
            // Wait with timeout
            let start = std::time::Instant::now();
            let timeout = Duration::from_secs(3);
            let mut child = child;
            loop {
                match child.try_wait() {
                    Ok(Some(_)) => break child.wait_with_output().ok(),
                    Ok(None) => {
                        if start.elapsed() > timeout {
                            let _ = child.kill();
                            return vec![];
                        }
                        std::thread::sleep(Duration::from_millis(50));
                    }
                    Err(_) => return vec![],
                }
            }
        }
        Err(_) => return vec![],
    };

    let output = match output {
        Some(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return vec![],
    };

    parse_runtime_output(&output, shell)
}

fn parse_runtime_output(output: &str, shell: &ShellType) -> Vec<RuntimeAlias> {
    let mut aliases = vec![];

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parsed = match shell {
            ShellType::Bash => parse_bash_runtime_line(line),
            ShellType::Zsh => parse_zsh_runtime_line(line),
            ShellType::Fish => parse_fish_runtime_line(line),
            ShellType::PowerShell => parse_powershell_runtime_line(line),
        };

        if let Some((name, command)) = parsed {
            aliases.push(RuntimeAlias {
                name,
                command,
                shell: shell.clone(),
            });
        }
    }

    aliases
}

// bash outputs: alias name='command'
fn parse_bash_runtime_line(line: &str) -> Option<(String, String)> {
    let line = line.strip_prefix("alias ")?;
    let eq_pos = line.find('=')?;
    let name = line[..eq_pos].to_string();
    let value = line[eq_pos + 1..].to_string();
    // Strip surrounding quotes
    let command = value
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .or_else(|| value.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
        .unwrap_or(&value)
        .to_string();
    Some((name, command))
}

// zsh outputs: name='command' or name=command
fn parse_zsh_runtime_line(line: &str) -> Option<(String, String)> {
    let eq_pos = line.find('=')?;
    let name = line[..eq_pos].to_string();
    let value = line[eq_pos + 1..].to_string();
    let command = value
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .or_else(|| value.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
        .unwrap_or(&value)
        .to_string();
    Some((name, command))
}

// fish outputs: alias name 'command'
fn parse_fish_runtime_line(line: &str) -> Option<(String, String)> {
    let line = line.strip_prefix("alias ")?;
    let space_pos = line.find(' ')?;
    let name = line[..space_pos].to_string();
    let rest = line[space_pos + 1..].trim();
    let command = rest
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .or_else(|| rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
        .unwrap_or(rest)
        .to_string();
    Some((name, command))
}

// PowerShell outputs: Name=Definition
fn parse_powershell_runtime_line(line: &str) -> Option<(String, String)> {
    let eq_pos = line.find('=')?;
    let name = line[..eq_pos].trim().to_string();
    let command = line[eq_pos + 1..].trim().to_string();
    if name.is_empty() || command.is_empty() {
        return None;
    }
    Some((name, command))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bash_runtime() {
        assert_eq!(
            parse_bash_runtime_line("alias gs='git status'"),
            Some(("gs".to_string(), "git status".to_string()))
        );
    }

    #[test]
    fn test_parse_zsh_runtime() {
        assert_eq!(
            parse_zsh_runtime_line("gs='git status'"),
            Some(("gs".to_string(), "git status".to_string()))
        );
        assert_eq!(
            parse_zsh_runtime_line("ll=ls"),
            Some(("ll".to_string(), "ls".to_string()))
        );
    }

    #[test]
    fn test_parse_fish_runtime() {
        assert_eq!(
            parse_fish_runtime_line("alias gs 'git status'"),
            Some(("gs".to_string(), "git status".to_string()))
        );
    }
}
