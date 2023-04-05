use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn process_file(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        // Do something with the line, which is a &str
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let file_name = &args[2];

    if command == "create" {
        // do something with file_name, which is a &str
    } else if command == "update" {
        // do something else with file_name
    }
    // and so on
}
