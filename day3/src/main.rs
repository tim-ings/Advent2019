use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

fn build_line(inst: &str, cur_x: &mut i32, cur_y: &mut i32) -> Line {
    let dir = &inst[0..1];
    let mag = &inst[1..inst.len()];
    let mag = mag.parse::<i32>().unwrap();
    let start_x = *cur_x;
    let start_y = *cur_y;
    match dir {
        "U" => *cur_y += mag,
        "D" => *cur_y -= mag,
        "L" => *cur_x -= mag,
        "R" => *cur_x += mag,
        _ => {
            println!("Bad line: {}", inst);
            panic!();
        },
    }
    return Line {
        start: Point {
            x: start_x,
            y: start_y,
        },
        end: Point {
            x: *cur_x,
            y: *cur_y,
        },
    }
}

fn parse_input(file_path: &str) -> Vec<Vec<Line>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut wires = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut wire = Vec::new();
        let mut cur_x = 0;
        let mut cur_y = 0;
        for s in line.split(",") {
            let line = build_line(s, &mut cur_x, &mut cur_y);
            println!("Built a line from ({}, {}) -> ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);
            wire.push(line);
        }
        wires.push(wire);
    }

    return wires;
}

fn within(test: i32, range_start: i32, range_end: i32) -> bool {
    if range_start < range_end {
        return test >= range_start && test <= range_end;
    } else {
        return test <= range_start && test >= range_end;
    }
}

fn is_horizontal(line: &Line) -> bool {
    return line.start.y == line.end.y;
}

fn is_vertical(line: &Line) -> bool {
    return line.start.x == line.end.x;
}

// this only works for perpendicular interections
fn line_intersects(line0: &Line, line1: &Line) -> (bool, Point) {
    if is_horizontal(line0) && is_vertical(line1) {
        // line0's y is constant and line1's x is constant
        if within(line1.start.x, line0.start.x, line0.end.x) && within(line0.start.y, line1.start.y, line1.end.y)  {
            println!("Intersection at ({}, {})!", line1.start.x, line0.start.y);
            return (true, Point { x: line1.start.x, y: line0.start.y });
        }
    } else if is_horizontal(line1) && is_vertical(line0) {
        // line0's x is constant and line1's y is constant
        if within(line0.start.x, line1.start.x, line1.end.x) && within(line1.start.y, line0.start.y, line0.end.y)  {
            println!("Intersection at ({}, {})!", line0.start.x, line1.start.y);
            return (true, Point { x: line0.start.x, y: line1.start.y });
        }
    }

    return (false, Point { x: 0, y: 0 });
}

fn manhattan_dist(p0: &Point, p1: &Point) -> i32 {
    return (p0.x - p1.x).abs() + (p0.y - p1.y).abs();
}

fn main() {
    // parse input into wires which consist of many lines
    let wires = parse_input("input.txt");
    

    // hard code for 2 wires only
    let wire0 = &wires[0];
    let wire1 = &wires[1];
    // check for intersections
    let mut intersects = Vec::new();
    for line0 in wire0.iter() {
        for line1 in wire1.iter() {
            let (res, intersect) = line_intersects(&line0, &line1);
            if res && intersect.x != 0 && intersect.y != 0 {
                intersects.push(intersect);
            }
        }
    }

    // find the closest intersection to the origin
    let origin = Point { x: 0, y: 0 };
    let mut shortest_dist = std::i32::MAX;
    for ip in intersects {
        let new_dist = manhattan_dist(&origin, &ip);
        if new_dist < shortest_dist {
            shortest_dist = new_dist;
        }
    }

    println!("Shortest distance is {}", shortest_dist);
}
