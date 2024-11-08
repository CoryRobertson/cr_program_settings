use serde::Deserialize;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use crate::serialization::deserialize_error::LoadSettingsError;
use crate::serialization::deserialize_error::LoadSettingsError::IOError;
use crate::serialization::deserialize_from_str;
use crate::SETTINGS_PATHS;

/// Loads a settings serialized file from `USER_HOME/crate_name/file_name`
pub fn load_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    match crate::get_user_home() {
        None => Err(LoadSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path = settings_path.join(PathBuf::from(file_name));
            match File::open(&settings_file_path) {
                Ok(mut file) => {
                    let mut file_data = String::new();
                    match file.read_to_string(&mut file_data) {
                        Ok(_) => {
                            match deserialize_from_str::<T>(&file_data) {
                                Ok(thing) => {
                                    {
                                        let mut lock = SETTINGS_PATHS.write().unwrap();
                                        if !lock.contains(&settings_file_path) {
                                            lock.push(settings_file_path);
                                        }
                                    }
                                    Ok(thing)
                                }
                                Err(err) => Err(LoadSettingsError::DeserializationError(err)),
                            }
                        },
                        Err(err) => Err(IOError(err)),
                    }
                }
                Err(err) => Err(IOError(err)),
            }
        }
    }
}

/// Loads a given settings file from the home directory and the given crate name.
/// Given `my_cool_rust_project`, the program would search in `/home/username/my_cool_rust_project` for a settings file
pub fn load_settings<T>(crate_name: &str) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    load_settings_with_filename(crate_name, format!("{}.ser", crate_name).as_str())
}