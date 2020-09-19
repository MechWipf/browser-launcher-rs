use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("Expected path to be a file")]
    RegistryNoFile,

    #[error("Expected path to be absolute")]
    RegistryNotAbsolute,

    #[error("Path is not valid. Symbolic links and network drives are not supported")]
    RegistryInvalid,

    #[error("Config invalid")]
    Config { source: toml::de::Error },

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}