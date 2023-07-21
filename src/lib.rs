//! cr_program_state is a library that simplifies saving a settings file for the program.
#![warn(missing_docs)]

use crate::LoadSettingsError::{DeserializationError, IOError};
use serde::Serialize;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use std::{fs, io};

/// Returns the users home as an optional using the "home" crate
pub fn get_user_home() -> Option<PathBuf> {
    home::home_dir()
}

#[macro_export]
/// Saves settings given a struct to save, to the home directory with a name matching the crate name
macro_rules! save_settings {
    ($t:expr) => {
        save_settings(env!("CARGO_CRATE_NAME"), &$t)
    };
}

#[macro_export]
/// Loads settings given a type to load, from the home directory with a name matching the crate name
macro_rules! load_settings {
    ($t:ty) => {
        load_settings::<$t>(env!("CARGO_CRATE_NAME"))
    };
}

#[macro_export]
/// Deletes settings located at the home directory with a name matching the crate name
macro_rules! delete_settings {
    () => {
        delete_settings(env!("CARGO_CRATE_NAME"))
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

/// Saves the settings file given in a directory named using the crate name
/// Given a struct and a crate name of "my_cool_rust_project", the program
/// would save it to /home/username/my_cool_rust_project/my_cool_rust_project.ser
pub fn save_settings<T>(crate_name: &str, settings: &T) -> Result<(), SaveSettingsError>
where
    T: Serialize,
{
    match get_user_home() {
        None => Err(SaveSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path =
                settings_path.join(PathBuf::from(format!("{}.ser", crate_name)));
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

/// Loads a given settings file from the home directory and the given crate name
/// Given "my_cool_rust_project", the program would search in /home/username/my_cool_rust_project for a settings file
pub fn load_settings<T>(crate_name: &str) -> Result<T, LoadSettingsError>
where
    T: serde::de::DeserializeOwned,
{
    match get_user_home() {
        None => Err(LoadSettingsError::FailedToGetUserHome),
        Some(home_dir) => {
            let settings_path = home_dir.join(PathBuf::from(crate_name));
            let settings_file_path =
                settings_path.join(PathBuf::from(format!("{}.ser", crate_name)));
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

/// Deletes the settings directory found in the <user home>/crate_name
/// e.g. /home/username/my_cool_project
pub fn delete_settings(crate_name: &str) -> io::Result<()> {
    let home_dir = get_user_home().unwrap();
    let settings_path = home_dir.join(PathBuf::from(crate_name));
    fs::remove_dir_all(settings_path)
}
