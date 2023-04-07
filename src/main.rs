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
            code,
            label,
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