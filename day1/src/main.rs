use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // open our input file
    let file = File::open("input.txt").unwrap();
    // create a buf reader to read it
    let reader = BufReader::new(file);

    let mut total_fuel:u32 = 0;

    for line in reader.lines() {
        let line:String = line.unwrap(); // we should never fail to read a line, or our input is bad
        let mass:u32 = line.parse::<u32>().unwrap(); // every line should be an int, or our input is bad
        let fuel_req:u32 = (mass / 3) - 2;
        total_fuel += fuel_req;
    }
    println!("Total Fuel Req = {}", total_fuel);
}
