use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    email: String,
    age: u32,
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_csv("data.csv") {
        eprintln!("Error reading CSV: {}", err);
    }
}
