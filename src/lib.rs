//! `cr_program_state` is a library that simplifies saving a settings file for the program.
#![warn(missing_docs)]

/// Global settings file path list, paths are added when successfully loaded, or when successfully saved.
pub static SETTINGS_PATHS: RwLock<Vec<PathBuf>> = RwLock::new(vec![]);

/// Module containing all logic related to serialization, including feature enabled serialization
pub mod serialization;

pub use delete::*;
pub use load::*;
pub use save::*;
pub use settings_container::SettingsContainer;
use std::path::PathBuf;
use std::sync::RwLock;

/// Module containing all deletion related code
pub mod delete;
/// Module containing all load related code
pub mod load;
/// Module containing all save related code
pub mod save;
/// Source code for the settings container.
pub mod settings_container;

/// Returns the users home as an optional using the "home" crate
pub fn get_user_home() -> Option<PathBuf> {
    home::home_dir()
}

#[cfg(any(
    all(
        feature = "json",
        any(feature = "yml", feature = "ron", feature = "toml")
    ),
    all(
        feature = "yml",
        any(feature = "json", feature = "ron", feature = "toml")
    ),
    all(
        feature = "ron",
        any(feature = "yml", feature = "json", feature = "toml")
    ),
    all(
        feature = "toml",
        any(feature = "yml", feature = "ron", feature = "json")
    ),
))]
compile_error!("Mutually exclusive features are being used for cr_program_settings, only use on serialization feature");

#[macro_export]
/// Saves settings given a struct to save, to the home directory with a name matching the crate name
///
/// Syntax:
///     save_settings!(settings_struct)
///     save_settings!(settings_struct, file_name)
///     save_settings!(settings_struct, file_name, folder_name)
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use cr_program_settings::*;
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
/// let loaded_settings = load_settings!().expect("Unable to read settings file");
///
/// assert_eq!(settings,loaded_settings);
///
/// save_settings!(settings,"cool_filename.ser").expect("Unable to save settings with specific filename");
///
/// // -- snip --
///
/// let specific_settings_loaded = load_settings!("cool_filename.ser").expect("Unable to load settings with specific filename");
///
/// assert_eq!(settings,specific_settings_loaded);
/// ```
macro_rules! save_settings {
    ($settings:expr) => {
        save_settings(env!("CARGO_CRATE_NAME"), &$settings)
    };
    ($settings: expr, $file_name: expr) => {
        save_settings_with_filename(env!("CARGO_CRATE_NAME"), &$file_name, &$settings)
    };
    ($settings: expr, $file_name: expr, $folder_name: expr) => {
        save_settings_with_filename($folder_name, &$file_name, &$settings)
    };
}

#[macro_export]
/// Loads settings given a type to load, from the home directory with a name matching the crate name
///
/// Syntax:
///     load_settings!(SETTINGS_TYPE)
///     load_settings!(SETTINGS_TYPE, file_name)
///     load_settings!(SETTINGS_TYPE, file_name,folder_name)
///
/// For more usage examples, see save_settings!() documentation.
/// ```
/// use serde::{Deserialize, Serialize};
/// use cr_program_settings::*;
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
/// save_settings!(settings,"odd_file_name.ser","unit_test_temp").expect("Unable to save settings to file");
///
/// let loaded_settings = load_settings!("odd_file_name.ser","unit_test_temp").expect("Failed to load settings file");
///
/// assert_eq!(settings,loaded_settings);
/// ```
macro_rules! load_settings {
    () => {
        load_settings(env!("CARGO_CRATE_NAME"))
    };
    ($file_name: expr) => {
        load_settings_with_filename(env!("CARGO_CRATE_NAME"), $file_name)
    };
    ($file_name: expr,$folder_name: expr) => {
        load_settings_with_filename($folder_name, $file_name)
    };
}

#[macro_export]
/// Deletes settings located at the home directory with a name matching the crate name
/// Syntax:
///     delete_settings!() // deletes file named: env!("CARGO_CRATE_NAME") file stored in the folder named: env!("CARGO_CRATE_NAME")
///     delete_settings!(file_name) // deletes the file named: file_name stored in the folder named: env!("CARGO_CRATE_NAME")
///     delete_settings!(file_name, folder_name) // deletes the file named: file_name stored in the folder named: folder_name
macro_rules! delete_settings {
    () => {
        delete_settings(env!("CARGO_CRATE_NAME"))
    };
    ($file_name: expr) => {
        delete_setting_file(env!("CARGO_CRATE_NAME"), $file_name)
    };
    ($file_name: expr,$folder_name: expr) => {
        delete_setting_file($folder_name, $file_name)
    };
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.is_ascii()
        && !name.contains(['\\', '/', ':', '*', '?', '"', '|', '<', '>', '𑼾', '®', 'ఒ'])
}
