use rustc_serialize::json;
use std::collections::HashMap;
use std::io::File;
use std::os;

pub fn load() -> HashMap<String, String> {
    let data = match File::open(&data_file_path()).read_to_string() {
        Ok(contents) => contents,
        Err(_) => "{}".to_string()
    };

    json::decode(data.as_slice()).unwrap()
}

pub fn save(rump_data: HashMap<String, String>) {
    let data = json::encode(&rump_data);

    File::create(&data_file_path()).write_str(data.as_slice()).ok();
}

fn data_file_path() -> Path {
    let home = os::homedir().unwrap();
    let mut path = Path::new(home);
    path.push(".rump");
    path
}
