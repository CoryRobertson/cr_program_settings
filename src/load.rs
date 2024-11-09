use crate::serialization::deserialize_error::LoadSettingsError;
use crate::serialization::deserialize_error::LoadSettingsError::IOError;
use crate::serialization::deserialize_from_str;
use crate::{valid_name, SETTINGS_PATHS};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Loads a settings serialized file from `USER_HOME/crate_name/file_name`
pub fn load_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    if !valid_name(crate_name) {
        return Err(LoadSettingsError::InvalidCrateName);
    }
    if !valid_name(file_name) {
        return Err(LoadSettingsError::InvalidFileName);
    }

    let home_dir = crate::get_user_home().ok_or(LoadSettingsError::FailedToGetUserHome)?;
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    let settings_file_path = settings_path.join(PathBuf::from(file_name));
    let mut file = File::open(&settings_file_path).map_err(|err| IOError(err))?;
    let mut file_data = String::new();

    let _ = file
        .read_to_string(&mut file_data)
        .map_err(|err| IOError(err))?;

    let deser = deserialize_from_str::<T>(&file_data)
        .map_err(|err| LoadSettingsError::DeserializationError(err))?;

    let mut lock = SETTINGS_PATHS
        .write()
        .map_err(|_| LoadSettingsError::MutexPoisoned)?;

    if !lock.contains(&settings_file_path) {
        lock.push(settings_file_path);
    }

    Ok(deser)
}

/// Loads a given settings file from the home directory and the given crate name.
/// Given `my_cool_rust_project`, the program would search in `/home/username/my_cool_rust_project` for a settings file
pub fn load_settings<T>(crate_name: &str) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    load_settings_with_filename(crate_name, format!("{}.ser", crate_name).as_str())
}
