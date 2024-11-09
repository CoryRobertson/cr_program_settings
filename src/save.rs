use crate::serialization::serialize_error::SaveSettingsError;
use crate::serialization::serialize_to_string;
use crate::{valid_name, SETTINGS_PATHS};
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Saves a serializable settings object to a given filename in `USER_HOME/crate_name/file_name`
pub fn save_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
    settings: &T,
) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    if !valid_name(crate_name) {
        return Err(SaveSettingsError::InvalidCrateName);
    }
    if !valid_name(file_name) {
        return Err(SaveSettingsError::InvalidFileName);
    }

    let home_dir = crate::get_user_home().ok_or(SaveSettingsError::FailedToGetUserHome)?;
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    let settings_file_path = settings_path.join(PathBuf::from(file_name));

    fs::create_dir_all(&settings_path).map_err(SaveSettingsError::IOError)?;
    let mut file =
        File::create(&settings_file_path).map_err(SaveSettingsError::IOError)?;
    let ser =
        serialize_to_string(&settings).map_err(SaveSettingsError::SerializationError)?;

    file
        .write_all(ser.as_bytes())
        .map_err(SaveSettingsError::IOError)?;

    let mut lock = SETTINGS_PATHS
        .write()
        .map_err(|_| SaveSettingsError::MutexPoisoned)?;
    lock.push(settings_file_path);

    Ok(())
}

/// Saves the settings file given in a directory named using the crate name
/// Given a struct and a crate name of `my_cool_rust_project`, the program
/// would save it to `/home/username/my_cool_rust_project/my_cool_rust_project.ser`
pub fn save_settings<T>(crate_name: &str, settings: &T) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    save_settings_with_filename(crate_name, format!("{}.ser", crate_name).as_str(), settings)
}
