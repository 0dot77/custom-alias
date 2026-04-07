use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Bash => write!(f, "bash"),
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::PowerShell => write!(f, "powershell"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedShell {
    pub shell_type: ShellType,
    pub binary_path: String,
    pub config_files: Vec<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub shell: ShellType,
    pub source_file: String,
    pub line_number: usize,
    pub group: Option<String>,
    pub is_managed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeAlias {
    pub name: String,
    pub command: String,
    pub shell: ShellType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AliasSource {
    ConfigFile { path: String, line: usize },
    RuntimeOnly,
    Both { path: String, line: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedAlias {
    pub name: String,
    pub command: String,
    pub shell: ShellType,
    pub source: AliasSource,
    pub group: Option<String>,
    pub is_managed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AliasInput {
    pub name: String,
    pub command: String,
    pub shell: ShellType,
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupInfo {
    pub path: String,
    pub shell: ShellType,
    pub created_at: String,
    pub original_file: String,
}
