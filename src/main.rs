mod csv_reader;

use std::collections::HashMap;

struct Specification {
    code: String,
    label: String,
    occurrences: i32
}

impl Specification {
    fn new(code: &str, label: &str, occurrences: i32) -> Specification {
        Specification {
            code: code.to_string(),
            label: label.to_string(),
            occurrences
        }
    }
}

fn main() {
    match csv_reader::read("test.csv") {
        Ok(map) => println!("{:?}", map),
        Err(e) => eprintln!("Error: {}", e),
    }
}