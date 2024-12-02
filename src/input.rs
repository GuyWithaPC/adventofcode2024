use std::fs;

pub fn load(input_dir: &str, day: u32, suffix: Option<&str>) -> String {
    load_raw(input_dir, day, suffix).trim().replace('\r', "")
}

pub fn load_raw(input_dir: &str, day: u32, suffix: Option<&str>) -> String {
    let file = format!("{}/{:02}{}.txt", input_dir, day, suffix.unwrap_or(""));
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}
