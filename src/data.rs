use rustc_serialize::json;
use std::collections::HashMap;
use std::old_io::File;
use std::env;

pub fn load() -> HashMap<String, String> {
    let data = match File::open(&data_file_path()).read_to_string() {
        Ok(contents) => contents,
        Err(_) => "{}".to_string()
    };

    json::decode(&data[]).unwrap()
}

pub fn save(rump_data: HashMap<String, String>) {
    let data = json::encode(&rump_data).unwrap();

    File::create(&data_file_path()).write_str(&data).ok();
}

fn data_file_path() -> Path {
    let home = env::home_dir().unwrap();
    let mut path = Path::new(home);
    path.push(".rump");
    path
}
