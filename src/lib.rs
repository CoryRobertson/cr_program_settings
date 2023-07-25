//! cr_program_state is a library that simplifies saving a settings file for the program.
#![warn(missing_docs)]

use crate::LoadSettingsError::{DeserializationError, IOError};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use std::{fs, io};

/// Source code for the settings container.
pub mod settings_container;

/// Returns the users home as an optional using the "home" crate
pub fn get_user_home() -> Option<PathBuf> {
    home::home_dir()
}

#[macro_export]
/// Saves settings given a struct to save, to the home directory with a name matching the crate name
/// ```
/// use serde::{Deserialize, Serialize};
/// use cr_program_settings::{load_settings, save_settings, save_settings_with_filename, load_settings_with_filename};
///
/// // create a struct we want to save, it needs to implement at a minimum of Serialize and Deserialize
/// #[derive(Serialize,Deserialize, PartialEq, Debug)]
/// struct Settings{
/// setting1: u32,
/// setting2: String,
/// setting3: Vec<bool>,
/// }
///
/// let settings = Settings{
///     setting1: 128,
///     setting2: "this is a cool setting struct".to_string(),
///     setting3: vec![false,true,false,false]
/// };
///
/// save_settings!(settings).expect("Settings were unable to be saved");
///
/// // -- snip --
///
/// let loaded_settings = load_settings!(Settings).expect("Unable to read settings file");
///
/// assert_eq!(settings,loaded_settings);
///
/// save_settings!(settings,"cool_filename.ser").expect("Unable to save settings with specific filename");
///
/// // -- snip --
///
/// let specific_settings_loaded = load_settings!(Settings,"cool_filename.ser").expect("Unable to load settings with specific filename");
///
/// assert_eq!(settings,specific_settings_loaded);
///
/// ```
macro_rules! save_settings {
    ($settings:expr) => {
        save_settings(env!("CARGO_CRATE_NAME"), &$settings)
    };
    ($settings: expr, $file_name: expr) => {
        save_settings_with_filename(env!("CARGO_CRATE_NAME"), &$file_name, &$settings)
    };
}

#[macro_export]
/// Loads settings given a type to load, from the home directory with a name matching the crate name
/// For usage examples, see save_settings!() documentation.
macro_rules! load_settings {
    ($setting_type:ty) => {
        load_settings::<$setting_type>(env!("CARGO_CRATE_NAME"))
    };
    ($setting_type:ty,$file_name: expr) => {
        load_settings_with_filename::<$setting_type>(env!("CARGO_CRATE_NAME"), $file_name)
    };
}

#[macro_export]
/// Deletes settings located at the home directory with a name matching the crate name
macro_rules! delete_settings {
    () => {
        delete_settings(env!("CARGO_CRATE_NAME"))
    };
    ($file_name: expr) => {
        delete_setting_file(env!("CARGO_CRATE_NAME"), $file_name)
    };
}

#[derive(Debug)]
/// An enum state representing the kinds of errors that saving settings has
pub enum SaveSettingsError {
    /// The library was unable to find the users home directory
    FailedToGetUserHome,
    /// The library encountered an io error when saving or creating the file or directory
    IOError(Error),
    /// The library encountered an error while serializing the struct
    SerializationError(toml::ser::Error),
}

/// Saves a serializable settings object to a given filename in USER_HOME/crate_name/file_name
pub fn save_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
    settings: &T,
) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    match get_user_home() {
        None => Err(SaveSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path = settings_path.join(PathBuf::from(file_name));
            match fs::create_dir_all(&settings_path) {
                Ok(_) => match File::create(settings_file_path) {
                    Ok(mut file) => match toml::to_string_pretty(&settings) {
                        Ok(serialized_data) => match file.write_all(serialized_data.as_bytes()) {
                            Ok(_) => Ok(()),
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
/// Given a struct and a crate name of "my_cool_rust_project", the program
/// would save it to /home/username/my_cool_rust_project/my_cool_rust_project.ser
pub fn save_settings<T>(crate_name: &str, settings: &T) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    save_settings_with_filename(crate_name, format!("{}.ser", crate_name).as_str(), settings)
}

#[derive(Debug)]
/// Enum state representing the possible errors that can occur when loading settings
pub enum LoadSettingsError {
    /// The library was unable to find the users home directory
    FailedToGetUserHome,
    /// The library encountered an io error while reading the file or accessing the directory
    IOError(Error),
    /// The library encountered an error while deserializing the settings file
    DeserializationError(toml::de::Error),
}

/// Loads a settings serialized file from USER_HOME/crate_name/file_name
pub fn load_settings_with_filename<T>(
    crate_name: &str,
    file_name: &str,
) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    match get_user_home() {
        None => Err(LoadSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path = settings_path.join(PathBuf::from(file_name));
            match File::open(settings_file_path) {
                Ok(mut file) => {
                    let mut file_data = String::new();
                    match file.read_to_string(&mut file_data) {
                        Ok(_) => match toml::from_str::<T>(&file_data) {
                            Ok(thing) => Ok(thing),
                            Err(err) => Err(DeserializationError(err)),
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
/// Given "my_cool_rust_project", the program would search in /home/username/my_cool_rust_project for a settings file
pub fn load_settings<T>(crate_name: &str) -> Result<T, LoadSettingsError>
where
    for<'a> T: Deserialize<'a>,
{
    load_settings_with_filename(crate_name, format!("{}.ser", crate_name).as_str())
}

/// Deletes the settings directory found in the <user home>/crate_name
/// e.g. /home/username/my_cool_project
pub fn delete_settings(crate_name: &str) -> io::Result<()> {
    let home_dir = get_user_home().unwrap();
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    fs::remove_dir_all(settings_path)
}

/// Deletes a specific settings file
pub fn delete_setting_file(crate_name: &str, file_name: &str) -> io::Result<()> {
    let home_dir = get_user_home().unwrap();
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    let settings_file = settings_path.join(file_name);
    fs::remove_file(settings_file)
}
