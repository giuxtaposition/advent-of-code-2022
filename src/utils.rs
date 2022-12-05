use std::fs;

pub fn read_input(input: &str) -> String {
    fs::read_to_string(input).expect("Should have been able to read the file")
}
