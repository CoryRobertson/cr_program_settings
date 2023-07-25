//! SettingsContainer source file
#![warn(missing_docs)]

use crate::{
    load_settings_with_filename, save_settings_with_filename, LoadSettingsError, SaveSettingsError,
};
use serde::{Deserialize, Serialize};

/// Struct that handles saving and loading.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SettingsContainer<T> {
    /// Generic settings inner field.
    settings: Option<T>,
    /// The name of the parent folder of where the file will be saved to.
    crate_name: String,
    /// The filename to save this struct
    file_name: String,
}

impl<T> SettingsContainer<T>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    /// Creates a new SettingsContainer
    pub fn new(content: T, crate_name: &str, file_name: &str) -> Self {
        Self {
            settings: Some(content),
            crate_name: crate_name.to_string(),
            file_name: file_name.to_string(),
        }
    }

    /// Gets the settings optional within the struct
    pub fn get_settings(&self) -> &Option<T> {
        &self.settings
    }

    /// Gets the mutable settings optional
    pub fn get_mut_settings(&mut self) -> Option<&mut T> {
        self.settings.as_mut()
    }

    /// Sets the settings optional within the struct
    pub fn set_settings(&mut self, settings: T) {
        self.settings = Some(settings);
    }

    /// Attempts to load a settings container, if it fails, it will return a default SettingsContainer
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use cr_program_settings::settings_container::SettingsContainer;
    ///
    /// #[derive(Serialize,Deserialize,PartialEq,Debug)]
    /// struct InnerStruct {
    /// field1: u32,
    /// field2: bool,
    /// field3: String,
    /// };
    ///
    /// let inner = InnerStruct{
    /// field1: 124,field2: true,field3: "cool struct data in a string!".to_string(),};
    ///
    /// let settings = SettingsContainer::new(inner,env!("CARGO_CRATE_NAME"),"doctest_save_settings.ser");
    ///
    /// let _ = settings.save().expect("Failed to save settings container to file");
    ///
    /// // This should load settings successfully
    /// let loaded_settings_success = SettingsContainer::<InnerStruct>::try_load_or_default(env!("CARGO_CRATE_NAME"),"doctest_save_settings.ser");
    /// assert_eq!(loaded_settings_success, settings);
    ///
    /// // This should fail, and resort to the default settings
    /// let loaded_settings_failed = SettingsContainer::<InnerStruct>::try_load_or_default(env!("CARGO_CRATE_NAME"),"not_a_settings_file.ser");
    /// assert_eq!(loaded_settings_failed, SettingsContainer::default(env!("CARGO_CRATE_NAME"),"not_a_settings_file.ser"));
    /// ```
    pub fn try_load_or_default(crate_name: &str, file_name: &str) -> Self {
        match SettingsContainer::<T>::load(crate_name, file_name) {
            Ok(settings_container) => settings_container,
            Err(_) => SettingsContainer::default(crate_name, file_name),
        }
    }

    /// Returns a default SettingsContainer
    pub fn default(crate_name: &str, file_name: &str) -> Self {
        Self {
            settings: None,
            crate_name: crate_name.to_string(),
            file_name: file_name.to_string(),
        }
    }

    /// Loads a settings container using a crate_name and file_name, returns a Ok(SettingsContainer) or Err(LoadSettingsError)
    /// For a unwrap_or_default style, use try_load_or_default()
    /// For example usage, see save() or try_load_or_default() documentation
    pub fn load(crate_name: &str, file_name: &str) -> Result<Self, LoadSettingsError> {
        load_settings_with_filename(crate_name, file_name)
    }

    /// Saves a settings container using its crate_name and file_name within the struct.
    /// ```
    /// use cr_program_settings::settings_container::SettingsContainer;
    ///
    /// let settings = SettingsContainer::new("some_cool_data".to_string(),env!("CARGO_CRATE_NAME"),"doctest_save_settings.ser");
    ///
    /// let _ = settings.save().expect("Failed to save settings container to file");
    ///
    /// let loaded_settings = SettingsContainer::load(env!("CARGO_CRATE_NAME"),"doctest_save_settings.ser").unwrap();
    ///
    /// assert_eq!(settings,loaded_settings);
    /// ```
    pub fn save(&self) -> Result<(), SaveSettingsError> {
        save_settings_with_filename(&self.crate_name, &self.file_name, self)
    }
}
