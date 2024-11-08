use std::{fs, io};
use std::path::PathBuf;
use crate::SETTINGS_PATHS;

/// Deletes the settings directory found in the `<user home>/crate_name`
/// e.g. `/home/username/my_cool_project`
pub fn delete_settings(crate_name: &str) -> io::Result<()> {
    let home_dir = crate::get_user_home().unwrap();
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    fs::remove_dir_all(&settings_path)?;
    SETTINGS_PATHS
        .write()
        .unwrap()
        .retain(|path| match path.parent() {
            None => true,
            Some(parent) => parent != settings_path,
        });
    Ok(())
}

/// Deletes a specific settings file
/// ```
/// use std::ffi::OsStr;
/// use serde::{Deserialize, Serialize};
/// use cr_program_settings::prelude::*;
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
pub fn delete_setting_file(crate_name: &str, file_name: &str) -> io::Result<()> {
    let home_dir = crate::get_user_home().unwrap();
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    let settings_file = settings_path.join(file_name);
    fs::remove_file(&settings_file)?;
    SETTINGS_PATHS
        .write()
        .unwrap()
        .retain(|path| path != &settings_file);
    Ok(())
}