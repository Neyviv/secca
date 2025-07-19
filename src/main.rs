use std::fmt::Display;

use csv::ReaderBuilder;
use rand::seq::IndexedRandom;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    quote: String,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" - {}", self.quote, self.name)
    }
}

fn print_random_quote() {
    let quotes_data = include_str!("../quotes.csv");
    let entries: Vec<&str> = quotes_data.lines().skip(1).collect();

    if let Some(entry) = entries.choose(&mut rand::rng()) {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(entry.as_bytes());
        for result in rdr.deserialize() {
            let parsed_quote: Record = result.expect("Deserializing failed");
            println!("{parsed_quote}");
        }
    }
}

fn main() {
    print_random_quote();
}
