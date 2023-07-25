# cr_program_settings
A library that simplifies the process of saving a struct containing the program settings to a file somewhere safe.
At the moment, the program allows you to give it a struct, and it will save it in the users home directory with the name of the program using the library.
Pretty minimal library that I plan on using for my other projects going forward.

###### Example usage:
```rust
use cr_program_settings::prelude::*;

 // create a struct we want to save, it needs to implement at a minimum of Serialize and Deserialize
 #[derive(Serialize,Deserialize, PartialEq, Debug)]
 struct Settings{
 setting1: u32,
 setting2: String,
 setting3: Vec<bool>,
 }

 let settings = Settings{
     setting1: 128,
     setting2: "this is a cool setting struct".to_string(),
     setting3: vec![false,true,false,false],
 };

 save_settings!(settings).expect("Settings were unable to be saved");

 // -- snip --

 let loaded_settings = load_settings!(Settings).expect("Unable to read settings file");

 assert_eq!(settings,loaded_settings);
```