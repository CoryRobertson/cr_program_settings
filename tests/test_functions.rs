use cr_program_settings::prelude::*;

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
