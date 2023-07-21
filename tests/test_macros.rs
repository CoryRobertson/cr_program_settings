use cr_program_settings::{delete_settings, load_settings, save_settings};
use serde::{Deserialize, Serialize};

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
            c: "asdad".to_string(),
            list: vec![
                "dsadasdsad49\"836521rf62%$^^%*%(^@".to_string(),
                "724\"'''''\"\"419".to_string(),
            ],
        },
        other_struct: OtherStruct {
            a: false,
            b: -390.724419,
            c: ("dsoicjsdoicsdoci".to_string(), -15),
        },
    };

    save_settings!(&t).unwrap();

    let loaded_settings = load_settings!(TestStruct).unwrap();
    assert_eq!(t, loaded_settings);

    delete_settings!().unwrap();
}
