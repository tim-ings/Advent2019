use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_fuel(mass: i32) -> i32 {
    // calculate the fuel requirement for the given mass
    let fuel_req: i32 = (mass / 3) - 2;

    // fuel add mass, so add the additional fuel requirment
    if fuel_req > 0 {
        return fuel_req + calc_fuel(fuel_req);
    } else {
        return 0;
    }
} 

fn main() {
    // open our input file
    let file = File::open("input.txt").unwrap();
    // create a buf reader to read it
    let reader = BufReader::new(file);

    let mut total_fuel: i32 = 0;

    for line in reader.lines() {
        // read the line and convert it to an int
        let line: String = line.unwrap(); // we should never fail to read a line, or our input is bad
        let mass: i32 = line.parse::<i32>().unwrap(); // every line should be an int, or our input is bad

        // calculate the fuel requirment
        let fuel_req: i32 = calc_fuel(mass);

        // add it to our total
        total_fuel += fuel_req;
    }
    println!("Total Fuel Req = {}", total_fuel);
}
