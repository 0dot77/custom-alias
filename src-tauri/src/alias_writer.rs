use crate::backup;
use crate::config_paths;
use crate::error::AppError;
use crate::types::{Alias, AliasInput, ShellType};

const MANAGED_START: &str = "# >>> custom-alias managed >>>";
const MANAGED_END: &str = "# <<< custom-alias managed <<<";

pub fn add_alias(input: &AliasInput) -> Result<Alias, AppError> {
    let target = config_paths::get_write_target(&input.shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: input.shell.to_string(),
        }
    })?;

    // Ensure parent directory exists
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create file if it doesn't exist
    if !target.exists() {
        std::fs::write(&target, "")?;
    }

    // Backup before modifying
    backup::create_backup(&target, &input.shell)?;

    let content = std::fs::read_to_string(&target)?;

    // Check for duplicate in managed section
    let alias_line = format_alias_line(&input.shell, &input.name, &input.command);
    if has_managed_alias(&content, &input.name, &input.shell) {
        return Err(AppError::DuplicateAlias {
            name: input.name.clone(),
            shell: input.shell.to_string(),
        });
    }

    let new_content = insert_into_managed_section(&content, &alias_line, &input.group, &input.shell);
    std::fs::write(&target, &new_content)?;

    let line_number = new_content
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains(&format!("alias {}", input.name)) || l.contains(&format!("Set-Alias -Name {}", input.name)))
        .map(|(i, _)| i + 1)
        .unwrap_or(0);

    Ok(Alias {
        name: input.name.clone(),
        command: input.command.clone(),
        shell: input.shell.clone(),
        source_file: target.to_string_lossy().to_string(),
        line_number,
        group: input.group.clone(),
        is_managed: true,
    })
}

pub fn update_alias(old_name: &str, input: &AliasInput) -> Result<Alias, AppError> {
    let target = config_paths::get_write_target(&input.shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: input.shell.to_string(),
        }
    })?;

    if !target.exists() {
        return Err(AppError::ConfigNotFound {
            path: target.to_string_lossy().to_string(),
        });
    }

    backup::create_backup(&target, &input.shell)?;

    let content = std::fs::read_to_string(&target)?;
    let old_pattern = alias_name_pattern(&input.shell, old_name);
    let new_line = format_alias_line(&input.shell, &input.name, &input.command);

    let mut new_lines = vec![];
    let mut found = false;
    let mut in_managed = false;

    for line in content.lines() {
        if line.contains(">>> custom-alias managed >>>") {
            in_managed = true;
        }
        if line.contains("<<< custom-alias managed <<<") {
            in_managed = false;
        }

        if in_managed && !found && line.contains(&old_pattern) {
            new_lines.push(new_line.clone());
            found = true;
        } else {
            new_lines.push(line.to_string());
        }
    }

    if !found {
        return Err(AppError::ManagedBlockError {
            detail: format!("Alias '{}' not found in managed section", old_name),
        });
    }

    let new_content = new_lines.join("\n");
    std::fs::write(&target, &new_content)?;

    let line_number = new_content
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains(&new_line))
        .map(|(i, _)| i + 1)
        .unwrap_or(0);

    Ok(Alias {
        name: input.name.clone(),
        command: input.command.clone(),
        shell: input.shell.clone(),
        source_file: target.to_string_lossy().to_string(),
        line_number,
        group: input.group.clone(),
        is_managed: true,
    })
}

pub fn delete_alias(name: &str, shell: &ShellType) -> Result<(), AppError> {
    let target = config_paths::get_write_target(shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: shell.to_string(),
        }
    })?;

    if !target.exists() {
        return Err(AppError::ConfigNotFound {
            path: target.to_string_lossy().to_string(),
        });
    }

    backup::create_backup(&target, shell)?;

    let content = std::fs::read_to_string(&target)?;
    let pattern = alias_name_pattern(shell, name);

    let mut new_lines = vec![];
    let mut found = false;
    let mut in_managed = false;

    for line in content.lines() {
        if line.contains(">>> custom-alias managed >>>") {
            in_managed = true;
        }
        if line.contains("<<< custom-alias managed <<<") {
            in_managed = false;
        }

        if in_managed && !found && line.contains(&pattern) {
            found = true;
            continue; // Skip this line (delete)
        }
        new_lines.push(line.to_string());
    }

    if !found {
        return Err(AppError::ManagedBlockError {
            detail: format!("Alias '{}' not found in managed section", name),
        });
    }

    std::fs::write(&target, new_lines.join("\n"))?;
    Ok(())
}

pub fn import_alias(name: &str, shell: &ShellType) -> Result<Alias, AppError> {
    let config_files = config_paths::get_config_files(shell);

    // Find the alias in any config file
    let mut found_line = None;
    let mut found_file = None;
    let pattern = alias_name_pattern(shell, name);

    for path in &config_files {
        if !path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(path)?;
        for (i, line) in content.lines().enumerate() {
            if line.contains(&pattern) && !is_in_managed_block(&content, i) {
                found_line = Some(line.to_string());
                found_file = Some(path.clone());
                break;
            }
        }
        if found_line.is_some() {
            break;
        }
    }

    let alias_line = found_line.ok_or_else(|| AppError::ManagedBlockError {
        detail: format!("Alias '{}' not found outside managed section", name),
    })?;
    let source = found_file.unwrap();

    // Parse the command from the line
    let command = extract_command_from_line(&alias_line, shell).unwrap_or_default();

    // Remove from original location
    backup::create_backup(&source, shell)?;
    let content = std::fs::read_to_string(&source)?;
    let new_content: String = content
        .lines()
        .filter(|l| !l.contains(&pattern) || is_comment(l))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&source, &new_content)?;

    // Add to managed section
    let input = crate::types::AliasInput {
        name: name.to_string(),
        command,
        shell: shell.clone(),
        group: None,
    };
    add_alias(&input)
}

// --- helpers ---

fn format_alias_line(shell: &ShellType, name: &str, command: &str) -> String {
    match shell {
        ShellType::Bash | ShellType::Zsh => format!("alias {}='{}'", name, command.replace('\'', "'\\''")),
        ShellType::Fish => format!("alias {} '{}'", name, command.replace('\'', "\\'")),
        ShellType::PowerShell => {
            if command.contains(' ') {
                format!("function {} {{ {} $args }}", name, command)
            } else {
                format!("Set-Alias -Name {} -Value {}", name, command)
            }
        }
    }
}

fn alias_name_pattern(shell: &ShellType, name: &str) -> String {
    match shell {
        ShellType::Bash | ShellType::Zsh => format!("alias {}=", name),
        ShellType::Fish => format!("alias {} ", name),
        ShellType::PowerShell => name.to_string(),
    }
}

fn has_managed_alias(content: &str, name: &str, shell: &ShellType) -> bool {
    let pattern = alias_name_pattern(shell, name);
    let mut in_managed = false;
    for line in content.lines() {
        if line.contains(">>> custom-alias managed >>>") {
            in_managed = true;
            continue;
        }
        if line.contains("<<< custom-alias managed <<<") {
            in_managed = false;
            continue;
        }
        if in_managed && line.contains(&pattern) {
            return true;
        }
    }
    false
}

fn is_in_managed_block(content: &str, target_line: usize) -> bool {
    let mut in_managed = false;
    for (i, line) in content.lines().enumerate() {
        if line.contains(">>> custom-alias managed >>>") {
            in_managed = true;
        }
        if line.contains("<<< custom-alias managed <<<") {
            in_managed = false;
        }
        if i == target_line {
            return in_managed;
        }
    }
    false
}

fn is_comment(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

fn extract_command_from_line(line: &str, shell: &ShellType) -> Option<String> {
    match shell {
        ShellType::Bash | ShellType::Zsh => {
            let after_eq = line.find('=').map(|i| &line[i + 1..])?;
            let trimmed = after_eq.trim();
            Some(
                trimmed
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                    .or_else(|| trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
                    .unwrap_or(trimmed)
                    .to_string(),
            )
        }
        ShellType::Fish => {
            let rest = line.trim().strip_prefix("alias ")?;
            let space = rest.find(' ')?;
            let cmd = rest[space + 1..].trim();
            Some(
                cmd.strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                    .unwrap_or(cmd)
                    .to_string(),
            )
        }
        ShellType::PowerShell => {
            if line.contains("Set-Alias") {
                line.split_whitespace().last().map(|s| s.to_string())
            } else if line.contains("function") {
                let start = line.find('{').map(|i| i + 1)?;
                let end = line.rfind('}')?;
                Some(line[start..end].trim().to_string())
            } else {
                None
            }
        }
    }
}

fn insert_into_managed_section(content: &str, alias_line: &str, group: &Option<String>, shell: &ShellType) -> String {
    let comment_char = match shell {
        ShellType::PowerShell => "#",
        _ => "#",
    };

    if content.contains(">>> custom-alias managed >>>") {
        // Insert before the end marker
        let mut lines: Vec<String> = vec![];
        let mut inserted = false;

        for line in content.lines() {
            if line.contains("<<< custom-alias managed <<<") && !inserted {
                if let Some(ref g) = group {
                    lines.push(format!("{} group: {}", comment_char, g));
                }
                lines.push(alias_line.to_string());
                inserted = true;
            }
            lines.push(line.to_string());
        }

        lines.join("\n")
    } else {
        // Create managed section at end of file
        let mut result = content.to_string();
        if !result.ends_with('\n') && !result.is_empty() {
            result.push('\n');
        }
        result.push('\n');
        result.push_str(MANAGED_START);
        result.push('\n');
        if let Some(ref g) = group {
            result.push_str(&format!("{} group: {}\n", comment_char, g));
        }
        result.push_str(alias_line);
        result.push('\n');
        result.push_str(MANAGED_END);
        result.push('\n');
        result
    }
}
