use rustc_serialize::json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn load() -> HashMap<String, String> {
    let mut data = String::new();

    match File::open(&data_file_path()) {
        Ok(mut file) => {
            file.read_to_string(&mut data).unwrap();
        },
        _ => {
            data.push_str("{}");
        }
    };

    json::decode(&data[..]).unwrap()
}

pub fn save(rump_data: HashMap<String, String>) {
    let data = json::encode(&rump_data).unwrap();
    let mut file = File::create(&data_file_path()).unwrap();
    write!(&mut file, "{}", data).unwrap();
}

fn data_file_path() -> PathBuf {
    let mut home = env::home_dir().unwrap();
    home.push(".rump");
    home
}
