use cr_program_settings::serialization::deserialize_error::LoadSettingsError;
use cr_program_settings::serialization::serialize_error::SaveSettingsError;
use cr_program_settings::serialization::DeleteSettingsError;
use cr_program_settings::{
    delete_setting_file, delete_settings, load_settings, load_settings_with_filename,
    save_settings, save_settings_with_filename,
};
use proptest::{prop_assert, prop_assert_eq, proptest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: f32,
    b: u32,
    c: String,
}

#[test]
fn test_functions() {
    let t = TestStruct {
        a: -10.0444,
        b: 0,
        c: "random text to save as a settings file".to_string(),
    };
    let crate_name = "cr_program_settings_2";
    save_settings(crate_name, &t).unwrap();

    let loaded_settings = load_settings::<TestStruct>(crate_name).unwrap();

    assert_eq!(t, loaded_settings);

    delete_settings(crate_name).unwrap();
}

proptest! {

    #[test]
    fn test_file_names_crate_names(file_name in "\\PC*", crate_name in "\\PC*") {

        let dummy_data = TestStruct {
            a: -10.0444,
            b: 0,
            c: "random text to save as a settings file".to_string(),
        };

        let res = save_settings_with_filename(&crate_name,&file_name, &dummy_data);

        match res.as_ref() {
            Ok(_) => {},
            Err(err) => {
                match err {
                    SaveSettingsError::InvalidCrateName | SaveSettingsError::InvalidFileName => {
                    },
                    _ => {
                        prop_assert!(false);
                    }
                }
            },
        }
        
        let load_res = load_settings_with_filename::<TestStruct>(&crate_name,&file_name);
        let delete_res = delete_setting_file(&crate_name,&file_name);
        
        if res.is_ok() {
            match load_res {
            Ok(loaded) => {
                prop_assert_eq!(loaded,dummy_data);
            },
            Err(e) => {
                match e {
                    LoadSettingsError::InvalidCrateName | LoadSettingsError::InvalidFileName => {
                    }
                    _ => {
                        prop_assert!(false, "{:?}", e);
                    }
                }
            }
            }

            

            match delete_res {
                Ok(_) => {},
                Err(err) => {
                    match err {
                        DeleteSettingsError::InvalidCrateName | DeleteSettingsError::InvalidFileName => {
                        },
                        _ => {
                            prop_assert!(false);
                        }
                    }
                },
            }
        }
        let _ = delete_settings!(&file_name,&crate_name);

    }

     #[test]
    fn test_crate_names(file_name in "\\PC*") {

        let dummy_data = TestStruct {
            a: -10.0444,
            b: 0,
            c: "random text to save as a settings file".to_string(),
        };

        let res = save_settings(&file_name, &dummy_data);

        match res.as_ref() {
            Ok(_) => {},
            Err(err) => {
                match err {
                    SaveSettingsError::InvalidCrateName | SaveSettingsError::InvalidFileName => {

                    },
                    _ => {
                        prop_assert!(false);
                    }
                }
            },
        }
        
        let load_res = load_settings::<TestStruct>(&file_name);
        let delete_res = delete_settings(&file_name);
        
        if res.is_ok() {
            match load_res {
            Ok(loaded) => {
                prop_assert_eq!(dummy_data,loaded);
            }
            Err(e) => {
                match e {
                    LoadSettingsError::InvalidCrateName | LoadSettingsError::InvalidFileName => {
                    }
                    _ => {
                        prop_assert!(false);
                    }
                }
            }
        }

        

        match delete_res {
            Ok(_) => {},
            Err(err) => {
                match err {
                    DeleteSettingsError::InvalidCrateName | DeleteSettingsError::InvalidFileName => {

                    },
                    _ => {
                        prop_assert!(false);
                    }
                }
            },
        }
        }
        let _ = delete_settings!(&file_name);
    }
}
