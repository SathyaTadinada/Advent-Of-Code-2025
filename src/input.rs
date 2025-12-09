use std::fs;

pub fn load(input_dir: &str, day: u32) -> String {
    load_raw(input_dir, day).trim().replace('\r', "")
}

pub fn load_raw(input_dir: &str, day: u32) -> String {
    let file = format!("{}/day{}.txt", input_dir, day);
    fs::read_to_string(&file)
        .unwrap_or_else(|_| panic!("Error reading file {file}."))
}
