use crate::types::{Alias, ShellType};
use regex::Regex;
use std::path::Path;

const MANAGED_START: &str = ">>> custom-alias managed >>>";
const MANAGED_END: &str = "<<< custom-alias managed <<<";

pub fn parse_config_file(path: &Path, shell: &ShellType) -> Vec<Alias> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    let source_file = path.to_string_lossy().to_string();
    parse_content(&content, shell, &source_file)
}

fn parse_content(content: &str, shell: &ShellType, source_file: &str) -> Vec<Alias> {
    match shell {
        ShellType::Bash | ShellType::Zsh => parse_bash_zsh(content, shell, source_file),
        ShellType::Fish => parse_fish(content, source_file),
        ShellType::PowerShell => parse_powershell(content, source_file),
    }
}

fn parse_bash_zsh(content: &str, shell: &ShellType, source_file: &str) -> Vec<Alias> {
    let re_single = Regex::new(r#"^\s*alias\s+([\w.:-]+)\s*=\s*'(.*)'\s*$"#).unwrap();
    let re_double = Regex::new(r#"^\s*alias\s+([\w.:-]+)\s*=\s*"(.*)"\s*$"#).unwrap();
    let re_unquoted = Regex::new(r#"^\s*alias\s+([\w.:-]+)\s*=\s*(\S+)\s*$"#).unwrap();
    let group_re = Regex::new(r"^#\s*group:\s*(.+)$").unwrap();

    let mut aliases = vec![];
    let mut in_managed = false;
    let mut current_group: Option<String> = None;

    for (line_num, line) in content.lines().enumerate() {
        if line.contains(MANAGED_START) {
            in_managed = true;
            current_group = None;
            continue;
        }
        if line.contains(MANAGED_END) {
            in_managed = false;
            current_group = None;
            continue;
        }

        if in_managed {
            if let Some(caps) = group_re.captures(line) {
                current_group = Some(caps[1].trim().to_string());
                continue;
            }
        }

        let alias = if let Some(caps) = re_single.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else if let Some(caps) = re_double.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else if let Some(caps) = re_unquoted.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else {
            None
        };

        if let Some((name, command)) = alias {
            aliases.push(Alias {
                name,
                command,
                shell: shell.clone(),
                source_file: source_file.to_string(),
                line_number: line_num + 1,
                group: if in_managed { current_group.clone() } else { None },
                is_managed: in_managed,
            });
        }
    }

    aliases
}

fn parse_fish(content: &str, source_file: &str) -> Vec<Alias> {
    let alias_re = Regex::new(r#"^\s*alias\s+([\w.:-]+)\s+['"]?(.+?)['"]?\s*$"#).unwrap();
    let abbr_re = Regex::new(r#"^\s*abbr\s+(?:-a\s+)?([\w.:-]+)\s+(.+)$"#).unwrap();
    let group_re = Regex::new(r"^#\s*group:\s*(.+)$").unwrap();

    let mut aliases = vec![];
    let mut in_managed = false;
    let mut current_group: Option<String> = None;

    for (line_num, line) in content.lines().enumerate() {
        if line.contains(MANAGED_START) {
            in_managed = true;
            current_group = None;
            continue;
        }
        if line.contains(MANAGED_END) {
            in_managed = false;
            current_group = None;
            continue;
        }

        if in_managed {
            if let Some(caps) = group_re.captures(line) {
                current_group = Some(caps[1].trim().to_string());
                continue;
            }
        }

        let alias = if let Some(caps) = alias_re.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else if let Some(caps) = abbr_re.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else {
            None
        };

        if let Some((name, command)) = alias {
            aliases.push(Alias {
                name,
                command,
                shell: ShellType::Fish,
                source_file: source_file.to_string(),
                line_number: line_num + 1,
                group: if in_managed { current_group.clone() } else { None },
                is_managed: in_managed,
            });
        }
    }

    aliases
}

fn parse_powershell(content: &str, source_file: &str) -> Vec<Alias> {
    let set_alias_re =
        Regex::new(r#"(?i)^\s*(?:Set|New)-Alias\s+(?:-Name\s+)?(\S+)\s+(?:-Value\s+)?(\S+)"#)
            .unwrap();
    let func_re = Regex::new(r#"^\s*function\s+(\S+)\s*\{([^}]*)\}"#).unwrap();
    let group_re = Regex::new(r"^#\s*group:\s*(.+)$").unwrap();

    let mut aliases = vec![];
    let mut in_managed = false;
    let mut current_group: Option<String> = None;

    for (line_num, line) in content.lines().enumerate() {
        if line.contains(MANAGED_START) {
            in_managed = true;
            current_group = None;
            continue;
        }
        if line.contains(MANAGED_END) {
            in_managed = false;
            current_group = None;
            continue;
        }

        if in_managed {
            if let Some(caps) = group_re.captures(line) {
                current_group = Some(caps[1].trim().to_string());
                continue;
            }
        }

        let alias = if let Some(caps) = set_alias_re.captures(line) {
            Some((caps[1].to_string(), caps[2].to_string()))
        } else if let Some(caps) = func_re.captures(line) {
            Some((caps[1].to_string(), caps[2].trim().to_string()))
        } else {
            None
        };

        if let Some((name, command)) = alias {
            aliases.push(Alias {
                name,
                command,
                shell: ShellType::PowerShell,
                source_file: source_file.to_string(),
                line_number: line_num + 1,
                group: if in_managed { current_group.clone() } else { None },
                is_managed: in_managed,
            });
        }
    }

    aliases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bash_simple() {
        let content = r#"
alias gs='git status'
alias gp="git push"
alias ll=ls
"#;
        let aliases = parse_content(content, &ShellType::Bash, "/test/.bashrc");
        assert_eq!(aliases.len(), 3);
        assert_eq!(aliases[0].name, "gs");
        assert_eq!(aliases[0].command, "git status");
        assert_eq!(aliases[1].name, "gp");
        assert_eq!(aliases[1].command, "git push");
        assert_eq!(aliases[2].name, "ll");
        assert_eq!(aliases[2].command, "ls");
    }

    #[test]
    fn test_parse_managed_block() {
        let content = r#"
alias existing='echo hello'
# >>> custom-alias managed >>>
# group: git
alias gs='git status'
alias gp='git push'
# <<< custom-alias managed <<<
alias other='echo other'
"#;
        let aliases = parse_content(content, &ShellType::Zsh, "/test/.zshrc");
        assert_eq!(aliases.len(), 4);
        assert!(!aliases[0].is_managed);
        assert!(aliases[1].is_managed);
        assert_eq!(aliases[1].group.as_deref(), Some("git"));
        assert!(aliases[2].is_managed);
        assert!(!aliases[3].is_managed);
    }

    #[test]
    fn test_parse_fish() {
        let content = r#"
alias gs 'git status'
abbr -a gp git push
"#;
        let aliases = parse_content(content, &ShellType::Fish, "/test/config.fish");
        assert_eq!(aliases.len(), 2);
        assert_eq!(aliases[0].name, "gs");
        assert_eq!(aliases[0].command, "git status");
        assert_eq!(aliases[1].name, "gp");
    }

    #[test]
    fn test_parse_powershell() {
        let content = r#"
Set-Alias -Name gs -Value Get-GitStatus
function gp { git push $args }
"#;
        let aliases = parse_content(content, &ShellType::PowerShell, "/test/profile.ps1");
        assert_eq!(aliases.len(), 2);
        assert_eq!(aliases[0].name, "gs");
        assert_eq!(aliases[0].command, "Get-GitStatus");
        assert_eq!(aliases[1].name, "gp");
    }
}
