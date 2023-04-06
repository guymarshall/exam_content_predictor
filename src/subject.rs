use std::collections::HashMap;

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