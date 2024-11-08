use std::io::Error;

#[derive(Debug)]
/// Enum state representing the possible errors that can occur when loading settings
pub enum LoadSettingsError {
    /// The library was unable to find the users home directory
    FailedToGetUserHome,
    /// The library encountered an io error while reading the file or accessing the directory
    IOError(Error),
    /// The library encountered an error while deserializing the settings file
    DeserializationError(DeserializationError),
}

#[derive(Debug)]
pub struct DeserializationError(
    #[cfg(feature = "ron")]
    pub ron::Error,
    #[cfg(feature = "toml")]
    pub toml::ser::Error,
    #[cfg(feature = "yml")]
    pub serde_yml::Error,
    #[cfg(feature = "json")]
    pub serde_json::Error,
);