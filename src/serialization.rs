use serde::{Deserialize, Serialize};
use crate::{DeserializationError, SerializationError};

/// Serialize a given struct to string using which ever serialization strategy feature is chosen for this crate
pub(crate) fn serialize_to_string<T: Serialize>(thing: &T) -> Result<String,SerializationError> {
    #[cfg(feature = "ron")]
    return ron::to_string(thing).map_err(|err| SerializationError(err));
    #[cfg(feature = "toml")]
    return toml::to_string_pretty(thing).map_err(|err| SerializationError(err));
    #[cfg(feature = "json")]
    return serde_json::to_string(thing).map_err(|err| SerializationError(err));
    #[cfg(feature = "yml")]
    return serde_yml::to_string(thing).map_err(|err| SerializationError(err));
}

/// Deserialize a given struct from string using which ever serialization strategy feature is chosen for this crate
pub(crate) fn deserialize_from_str<T>(thing: &str) -> Result<T,DeserializationError>
where
        for<'a> T: Deserialize<'a>, {
    #[cfg(feature = "ron")]
    return ron::from_str(thing).map_err(|err| DeserializationError(err.into()));
    #[cfg(feature = "toml")]
    return toml::from_str(thing).map_err(|err| crate::DeserializationError(err));
    #[cfg(feature = "json")]
    return serde_json::from_str(thing).map_err(|err| crate::DeserializationError(err));
    #[cfg(feature = "yml")]
    return serde_yml::from_str(thing).map_err(|err| crate::DeserializationError(err));
}