use std::fs;

pub fn get_file_contents(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .unwrap()
        .replace("\r\n", "\n")
        .replace("\r", "\n")
}
