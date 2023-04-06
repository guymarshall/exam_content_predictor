mod subject;
mod csv_reader;

fn main() {
    match csv_reader::read("test.csv") {
        Ok(map) => println!("{:?}", map),
        Err(e) => eprintln!("Error: {}", e),
    }
}