use crate::types::{Alias, AliasSource, MergedAlias, RuntimeAlias};
use std::collections::HashMap;

pub fn merge_aliases(file_aliases: Vec<Alias>, runtime_aliases: Vec<RuntimeAlias>) -> Vec<MergedAlias> {
    let mut merged: HashMap<String, MergedAlias> = HashMap::new();

    // First, add all file-parsed aliases
    for alias in &file_aliases {
        merged.insert(
            alias.name.clone(),
            MergedAlias {
                name: alias.name.clone(),
                command: alias.command.clone(),
                shell: alias.shell.clone(),
                source: AliasSource::ConfigFile {
                    path: alias.source_file.clone(),
                    line: alias.line_number,
                },
                group: alias.group.clone(),
                is_managed: alias.is_managed,
            },
        );
    }

    // Then merge runtime aliases
    for rt_alias in &runtime_aliases {
        if let Some(existing) = merged.get_mut(&rt_alias.name) {
            // Found in both file and runtime -> upgrade to Both
            if let AliasSource::ConfigFile { ref path, line } = existing.source {
                existing.source = AliasSource::Both {
                    path: path.clone(),
                    line,
                };
            }
            // If runtime command differs, prefer runtime (it's the actual active one)
            if existing.command != rt_alias.command {
                existing.command = rt_alias.command.clone();
            }
        } else {
            // Only in runtime
            merged.insert(
                rt_alias.name.clone(),
                MergedAlias {
                    name: rt_alias.name.clone(),
                    command: rt_alias.command.clone(),
                    shell: rt_alias.shell.clone(),
                    source: AliasSource::RuntimeOnly,
                    group: None,
                    is_managed: false,
                },
            );
        }
    }

    let mut result: Vec<MergedAlias> = merged.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShellType;

    #[test]
    fn test_merge_both() {
        let file_aliases = vec![Alias {
            name: "gs".to_string(),
            command: "git status".to_string(),
            shell: ShellType::Zsh,
            source_file: "/home/.zshrc".to_string(),
            line_number: 10,
            group: Some("git".to_string()),
            is_managed: true,
        }];
        let runtime_aliases = vec![RuntimeAlias {
            name: "gs".to_string(),
            command: "git status".to_string(),
            shell: ShellType::Zsh,
        }];

        let merged = merge_aliases(file_aliases, runtime_aliases);
        assert_eq!(merged.len(), 1);
        assert!(matches!(merged[0].source, AliasSource::Both { .. }));
    }

    #[test]
    fn test_merge_runtime_only() {
        let file_aliases = vec![];
        let runtime_aliases = vec![RuntimeAlias {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
            shell: ShellType::Zsh,
        }];

        let merged = merge_aliases(file_aliases, runtime_aliases);
        assert_eq!(merged.len(), 1);
        assert!(matches!(merged[0].source, AliasSource::RuntimeOnly));
        assert!(!merged[0].is_managed);
    }
}
