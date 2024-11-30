mod identity;

use std::fs;

fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
