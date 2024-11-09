use crate::serialization::DeleteSettingsError;
use crate::{valid_name, SETTINGS_PATHS};
use std::fs;
use std::path::PathBuf;

/// Deletes the settings directory found in the `<user home>/crate_name`
/// e.g. `/home/username/my_cool_project`
pub fn delete_settings(crate_name: &str) -> Result<(), DeleteSettingsError> {
    delete_setting_file(crate_name, format!("{}.ser", crate_name).as_str())
}

/// Deletes a specific settings file
/// ```
/// use std::ffi::OsStr;
/// use serde::{Deserialize, Serialize};
/// use cr_program_settings::*;
/// #[derive(Serialize,Deserialize)]
/// struct TestStruct {field1: u32}
///
/// let s = TestStruct{field1: 6};
///
/// let sn = "settings_file_978.ser";
/// save_settings!(s,sn);
/// assert!(SETTINGS_PATHS.read().unwrap().iter().any(|path| {
/// match path.file_name() {
/// None => { false }
/// Some(file_name) => { file_name == sn }
/// }
/// }));
///
/// delete_settings!(sn);
/// assert!(!SETTINGS_PATHS.read().unwrap().iter().any(|path| {
/// match path.file_name() {
/// None => { false }
/// Some(file_name) => { file_name == sn }
/// }
/// }));
///
///
/// ```
pub fn delete_setting_file(crate_name: &str, file_name: &str) -> Result<(), DeleteSettingsError> {
    if !valid_name(crate_name) {
        return Err(DeleteSettingsError::InvalidCrateName);
    }
    if !valid_name(file_name) {
        return Err(DeleteSettingsError::InvalidFileName);
    }

    let home_dir = crate::get_user_home().ok_or(DeleteSettingsError::FailedToGetUserHome)?;
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    let settings_file = settings_path.join(file_name);
    fs::remove_file(&settings_file).map_err(|err| DeleteSettingsError::IOError(err))?;
    println!("{},{}", crate_name, file_name);
    SETTINGS_PATHS
        .write()
        .map_err(|_| DeleteSettingsError::MutexPoisoned)?
        .retain(|path| path != &settings_file);
    Ok(())
}
