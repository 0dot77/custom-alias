use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Shell config file not found: {path}")]
    ConfigNotFound { path: String },

    #[error("Failed to parse config: {detail}")]
    ParseError { detail: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Alias '{name}' already exists in {shell}")]
    DuplicateAlias { name: String, shell: String },

    #[error("Managed block error: {detail}")]
    ManagedBlockError { detail: String },

    #[error("Shell not found: {shell}")]
    ShellNotFound { shell: String },

    #[error("Runtime query failed: {detail}")]
    RuntimeError { detail: String },
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
