use data;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn delete(key: &str) {
    let mut rump_data = data::load();

    rump_data.remove(key);

    data::save(rump_data);
}

pub fn get(key: &str) -> Option<String> {
    let rump_data = data::load();

    match rump_data.get(key) {
        Some(value) => Some(value.clone()),
        None => None
    }
}

pub fn set(key: &str, value: &str) {
    let mut rump_data = data::load();

    rump_data.insert(key.to_string(), value.to_string());

    data::save(rump_data);
}

pub fn version() -> String {
    format!("rump {}", VERSION)
}
