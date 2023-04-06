use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::{Reader, ReaderBuilder, StringRecord};

pub fn read(file_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file: File = File::open(file_path)?;
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut records: Vec<StringRecord> = Vec::new();
    for result in reader.records() {
        let record: StringRecord = result?;
        records.push(record);
    }

    let map: HashMap<String, String> = records.into_iter().fold(HashMap::new(), |mut accumulator: HashMap<String, String>, record: StringRecord| {
        let key: String = record[0].to_owned();
        let value: String = record[1].to_owned();
        accumulator.insert(key, value);
        accumulator
    });
    Ok(map)
}