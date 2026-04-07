use crate::backup;
use crate::config_paths;
use crate::error::AppError;
use crate::types::{Alias, AliasInput, ShellType};
use regex::Regex;

const MANAGED_START: &str = "# >>> custom-alias managed >>>";
const MANAGED_END: &str = "# <<< custom-alias managed <<<";

/// Validate alias name: only alphanumeric, underscore, dot, colon, hyphen allowed
fn validate_alias_name(name: &str) -> Result<(), AppError> {
    let re = Regex::new(r"^[\w.:-]+$").unwrap();
    if name.is_empty() || !re.is_match(name) {
        return Err(AppError::ParseError {
            detail: format!("Invalid alias name '{}': only alphanumeric, _, ., :, - allowed", name),
        });
    }
    Ok(())
}

/// Build a regex that matches the alias definition line for a given name
fn alias_line_regex(shell: &ShellType, name: &str) -> Regex {
    let escaped = regex::escape(name);
    match shell {
        ShellType::Bash | ShellType::Zsh => {
            Regex::new(&format!(r"^\s*alias\s+{}=", escaped)).unwrap()
        }
        ShellType::Fish => {
            Regex::new(&format!(r"^\s*alias\s+{}\s", escaped)).unwrap()
        }
        ShellType::PowerShell => {
            Regex::new(&format!(
                r"(?i)(?:^\s*(?:Set|New)-Alias\s+(?:-Name\s+)?{}\s|^\s*function\s+{}\s*\{{)",
                escaped, escaped
            ))
            .unwrap()
        }
    }
}

fn line_matches(shell: &ShellType, name: &str, line: &str) -> bool {
    alias_line_regex(shell, name).is_match(line)
}

/// Detect line ending style of content
fn detect_line_ending(content: &str) -> &'static str {
    if content.contains("\r\n") { "\r\n" } else { "\n" }
}

pub fn add_alias(input: &AliasInput) -> Result<Alias, AppError> {
    validate_alias_name(&input.name)?;

    let target = config_paths::get_write_target(&input.shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: input.shell.to_string(),
        }
    })?;

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !target.exists() {
        std::fs::write(&target, "")?;
    }

    backup::create_backup(&target, &input.shell)?;

    let content = std::fs::read_to_string(&target)?;
    let line_ending = detect_line_ending(&content);

    if has_managed_alias(&content, &input.name, &input.shell) {
        return Err(AppError::DuplicateAlias {
            name: input.name.clone(),
            shell: input.shell.to_string(),
        });
    }

    let alias_line = format_alias_line(&input.shell, &input.name, &input.command);
    let new_content = insert_into_managed_section(&content, &alias_line, &input.group, line_ending);
    std::fs::write(&target, &new_content)?;

    let line_number = new_content
        .lines()
        .enumerate()
        .find(|(_, l)| line_matches(&input.shell, &input.name, l))
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
    validate_alias_name(&input.name)?;

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
    let line_ending = detect_line_ending(&content);
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

        if in_managed && !found && line_matches(&input.shell, old_name, line) {
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

    let mut new_content = new_lines.join(line_ending);
    if content.ends_with('\n') || content.ends_with("\r\n") {
        new_content.push_str(line_ending);
    }
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
    let line_ending = detect_line_ending(&content);

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

        if in_managed && !found && line_matches(shell, name, line) {
            found = true;
            continue;
        }
        new_lines.push(line.to_string());
    }

    if !found {
        return Err(AppError::ManagedBlockError {
            detail: format!("Alias '{}' not found in managed section", name),
        });
    }

    let mut new_content = new_lines.join(line_ending);
    if content.ends_with('\n') || content.ends_with("\r\n") {
        new_content.push_str(line_ending);
    }
    std::fs::write(&target, &new_content)?;
    Ok(())
}

/// Delete an alias from a config file by file path and line number.
/// Used for removing external/unmanaged aliases (e.g. corrupted or unwanted entries).
/// Line-based deletion avoids regex issues with corrupted alias names.
pub fn delete_external_alias(file_path: &str, line: usize, shell: &ShellType) -> Result<(), AppError> {
    let path = std::path::Path::new(file_path);

    if !path.exists() {
        return Err(AppError::ConfigNotFound {
            path: file_path.to_string(),
        });
    }

    let content = std::fs::read_to_string(path)?;
    let line_ending = detect_line_ending(&content);
    let lines: Vec<&str> = content.lines().collect();

    // line is 1-based from the parser
    if line == 0 || line > lines.len() {
        return Err(AppError::ManagedBlockError {
            detail: format!("Line {} is out of range for {}", line, file_path),
        });
    }

    backup::create_backup(path, shell)?;

    let new_lines: Vec<&str> = lines
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != line - 1) // convert 1-based to 0-based
        .map(|(_, l)| *l)
        .collect();

    let mut new_content = new_lines.join(line_ending);
    if content.ends_with('\n') || content.ends_with("\r\n") {
        new_content.push_str(line_ending);
    }
    std::fs::write(path, &new_content)?;
    Ok(())
}

/// Suppress a runtime-only alias by adding `unalias <name>` to the managed section.
/// This effectively removes plugin aliases on next shell startup.
pub fn suppress_alias(name: &str, shell: &ShellType) -> Result<(), AppError> {
    let target = config_paths::get_write_target(shell).ok_or_else(|| {
        AppError::ShellNotFound {
            shell: shell.to_string(),
        }
    })?;

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !target.exists() {
        std::fs::write(&target, "")?;
    }

    backup::create_backup(&target, shell)?;

    let content = std::fs::read_to_string(&target)?;
    let line_ending = detect_line_ending(&content);

    let unalias_line = match shell {
        ShellType::Bash | ShellType::Zsh => format!("unalias {} 2>/dev/null || true", name),
        ShellType::Fish => format!("functions -e {}", name),
        ShellType::PowerShell => format!("Remove-Alias -Name {} -ErrorAction SilentlyContinue", name),
    };

    let new_content = insert_into_managed_section(&content, &unalias_line, &None, line_ending);
    std::fs::write(&target, &new_content)?;
    Ok(())
}

pub fn import_alias(name: &str, shell: &ShellType) -> Result<Alias, AppError> {
    let config_files = config_paths::get_config_files(shell);

    let mut found_line = None;
    let mut found_file = None;
    let mut found_line_num = None;

    for path in &config_files {
        if !path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(path)?;
        for (i, line) in content.lines().enumerate() {
            if line_matches(shell, name, line) && !is_in_managed_block(&content, i) {
                found_line = Some(line.to_string());
                found_file = Some(path.clone());
                found_line_num = Some(i);
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
    let target_line_num = found_line_num.unwrap();

    let command = extract_command_from_line(&alias_line, shell).unwrap_or_default();

    // Remove from original location by exact line number
    backup::create_backup(&source, shell)?;
    let content = std::fs::read_to_string(&source)?;
    let line_ending = detect_line_ending(&content);
    let new_content: String = content
        .lines()
        .enumerate()
        .filter(|(i, _)| *i != target_line_num)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join(line_ending);
    std::fs::write(&source, &new_content)?;

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

fn has_managed_alias(content: &str, name: &str, shell: &ShellType) -> bool {
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
        if in_managed && line_matches(shell, name, line) {
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

fn insert_into_managed_section(content: &str, alias_line: &str, group: &Option<String>, line_ending: &str) -> String {
    if content.contains(">>> custom-alias managed >>>") {
        let mut lines: Vec<String> = vec![];
        let mut inserted = false;

        for line in content.lines() {
            if line.contains("<<< custom-alias managed <<<") && !inserted {
                if let Some(ref g) = group {
                    lines.push(format!("# group: {}", g));
                }
                lines.push(alias_line.to_string());
                inserted = true;
            }
            lines.push(line.to_string());
        }

        let mut result = lines.join(line_ending);
        if content.ends_with('\n') || content.ends_with("\r\n") {
            result.push_str(line_ending);
        }
        result
    } else {
        let mut result = content.to_string();
        if !result.ends_with('\n') && !result.is_empty() {
            result.push_str(line_ending);
        }
        result.push_str(line_ending);
        result.push_str(MANAGED_START);
        result.push_str(line_ending);
        if let Some(ref g) = group {
            result.push_str(&format!("# group: {}", g));
            result.push_str(line_ending);
        }
        result.push_str(alias_line);
        result.push_str(line_ending);
        result.push_str(MANAGED_END);
        result.push_str(line_ending);
        result
    }
}
