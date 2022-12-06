use std::{env, fs};

pub fn read_input(input: &str) -> String {
    let docs_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .replace("rust", "days_docs");

    fs::read_to_string(format!("{docs_dir}/{input}"))
        .expect("Should have been able to read the file")
}
