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

struct Subject {
    name: String,
    specification: HashMap<String, String>
}

impl Subject {
    fn new(subject_name: &str, subject_specification: HashMap<&str, &str>) -> Subject {
        let specification: HashMap<String, String> = subject_specification
            .iter()
            .map(|(&key, &value)| (key.to_string(), value.to_string()))
            .collect();

        Subject {
            name: subject_name.to_string(),
            specification,
        }
    }
}

fn main() {
    match csv_reader::read("test.csv") {
        Ok(map) => println!("{:?}", map),
        Err(e) => eprintln!("Error: {}", e),
    }
}