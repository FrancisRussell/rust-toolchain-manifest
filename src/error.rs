use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Manifest had incorect structure: {0}")]
    IncorrectManifestStructure(String),

    #[error("A value could be parsed as a date: {0}")]
    InvalidDate(#[from] chrono::ParseError),
}