use std::io::Error;

#[derive(Debug)]
/// An enum state representing the kinds of errors that saving settings has
pub enum SaveSettingsError {
    /// The library was unable to find the users home directory
    FailedToGetUserHome,
    /// The library encountered an io error when saving or creating the file or directory
    IOError(Error),
    /// The library encountered an error while serializing the struct
    SerializationError(SerializationError),
}

#[derive(Debug)]
pub struct SerializationError(
    #[cfg(feature = "ron")]
    pub ron::Error,
    #[cfg(feature = "toml")]
    pub toml::ser::Error,
    #[cfg(feature = "yml")]
    pub serde_yml::Error,
    #[cfg(feature = "json")]
    pub serde_json::Error,
);