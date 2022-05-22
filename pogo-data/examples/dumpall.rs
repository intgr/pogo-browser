use pogo_data::parse_json;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("data/latest.json").expect("Cannot open latest.json");

    let data = parse_json(BufReader::new(file)).expect("Error parsing latest.json");

    println!("{:#?}", data);
}
