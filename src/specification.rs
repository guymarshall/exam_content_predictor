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
}//use this struct in the subject struct instead of the hashmap