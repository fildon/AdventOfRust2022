use std::fs;
pub mod day01caloriecounting;
pub mod day02rockpaperscissors;
pub mod day03rucksackreorganization;
pub mod utils {
    use super::*;
    pub fn get_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path)
            .unwrap()
            .replace("\r\n", "\n")
            .replace("\r", "\n")
    }
    pub fn get_lines(file_path: &str) -> Vec<String> {
        get_file_contents(file_path)
            .lines()
            .map(ToOwned::to_owned)
            .collect()
    }
}
