extern crate petgraph;

use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graphmap::DiGraphMap;

fn pack_id(s: &str) -> u32 {
    let mut id: u32 = 0;
    for (i, c) in s.as_bytes().iter().enumerate() {
        let c: u32 = *c as u32;
        id |= c << (24 - (i * 8));
    }
    return id;
}

fn unpack_id(id: u32) -> String {
    let mut s = String::new();
    s.push((id >> 24) as u8 as char);
    s.push((id >> 16) as u8 as char);
    s.push((id >> 8) as u8 as char);
    s.push(id as u8 as char);
    return s;
}

fn parse_input(file_path: &str) -> DiGraphMap<u32, u32> {
    let mut bodies: DiGraphMap<u32, u32> = DiGraphMap::new();
    let file = File::open(file_path).expect("Unable to read file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let rel: Vec<String> = line.expect("unable to read line").split(")").map(|s| s.to_string()).collect();
        bodies.add_edge(pack_id(&rel[1]), pack_id(&rel[0]), 1);
    }
    return bodies;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn orbit_count(univ: &DiGraphMap<u32, u32>, n: u32) -> u32 {
    // pack_id("COM") -> 1129270528
    if n == 1129270528 {
        return 0;
    }
    for nb in univ.neighbors(n) {
        return 1 + orbit_count(univ, nb);
    }
    return 0;
}

fn main() {
    let univ = parse_input("input.txt");
    println!("{}", pack_id("COM"));
    println!("Body Count {}", univ.node_count());
    println!("Direct Orbit Count {}", univ.edge_count());
    let mut oc = 0;
    for n in univ.nodes() {
        oc += orbit_count(&univ, n);
    }
    println!("Direct + Indirect Orbit Count: {}", oc);
}
