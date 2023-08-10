use serde::{Deserialize, Serialize};
use cr_program_settings::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
struct TestStruct {
    settings: Settings,
    other_struct: OtherStruct,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
struct Settings {
    #[serde(default)]
    a: i32,
    #[serde(default)]
    b: bool,
    #[serde(default)]
    c: String,

    list: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
struct OtherStruct {
    a: bool,
    b: f32,
    c: (String, i32),
}

#[test]
fn test_primary_macros() {
    let t = TestStruct {
        settings: Settings {
            a: 17,
            b: true,
            c: "settings data".to_string(),
            list: vec![
                "random data incoming!!!\"836521rf62%$^^%*%(^@".to_string(),
                "724\"'''''\"\"419".to_string(),
            ],
        },
        other_struct: OtherStruct {
            a: false,
            b: -390.724419,
            c: ("random test data$$!#".to_string(), -15),
        },
    };

    save_settings!(&t).unwrap();

    let loaded_settings = load_settings!(TestStruct).unwrap();
    assert_eq!(t, loaded_settings);

    delete_settings!().unwrap();
}

#[test]
fn test_filename_macros() {
    let s = TestStruct {
        settings: Settings {
            a: 4,
            b: true,
            c: "sa51das651sad65a1s65s1a65d".to_string(),
            list: vec![
                "1/e2'1]e.12][1.e1e1e1@!#!@#!%^%&^&(*()*()()_)_49898-8-*8-*/".to_string(),
                "d65w4d163489wd41a68d4".to_string(),
                "543468g494h4964e".to_string(),
            ],
        },
        other_struct: Default::default(),
    };

    let file_name = "test_macro_settings";

    let _ = save_settings!(&s, file_name).unwrap();

    let loaded_settings = load_settings!(TestStruct, file_name).unwrap();

    assert_eq!(loaded_settings, s);

    delete_settings!(file_name).unwrap();
}
