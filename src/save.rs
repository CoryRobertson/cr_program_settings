use serde::Serialize;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::serialization::serialize_error::SaveSettingsError;
use crate::serialization::serialize_to_string;
use crate::SETTINGS_PATHS;

/// Saves a serializable settings object to a given filename in `USER_HOME/crate_name/file_name`
pub fn save_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
    settings: &T,
) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    match crate::get_user_home() {
        None => Err(SaveSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path = settings_path.join(PathBuf::from(file_name));
            match fs::create_dir_all(&settings_path) {
                Ok(_) => match File::create(&settings_file_path) {
                    Ok(mut file) => match serialize_to_string(&settings) {
                        Ok(serialized_data) => match file.write_all(serialized_data.as_bytes()) {
                            Ok(_) => {
                                {
                                    let mut lock = SETTINGS_PATHS.write().unwrap();
                                    lock.push(settings_file_path);
                                }
                                Ok(())
                            }
                            Err(err) => Err(SaveSettingsError::IOError(err)),
                        },
                        Err(err) => Err(SaveSettingsError::SerializationError(err)),
                    },
                    Err(err) => Err(SaveSettingsError::IOError(err)),
                },
                Err(err) => Err(SaveSettingsError::IOError(err)),
            }
        }
    }
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