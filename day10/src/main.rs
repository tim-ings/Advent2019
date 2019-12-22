use std::fs;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    asteroids: Vec<Point>,
}

impl Grid {
    fn new(file_path: &str) -> Self {
        // read input from file
        let lines: Vec<String> = fs::read_to_string(file_path)
                                .unwrap()
                                .trim()
                                .lines()
                                .map(|s| s.to_string())
                                .collect();

        let mut asteroids = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ast) in line.chars().enumerate() {
                if ast == '#' {
                    asteroids.push(Point { x: x as i32, y: y as i32 });
                }
            }
        }

        let width = lines[0].len();
        let height = lines.len();

        return Grid {
            width: width,
            height: height,
            asteroids: asteroids,
        };
    }
}

fn angle(src: &Point, dest: &Point) -> f32 {
    let x = (dest.x - src.x) as f32;
    let y = (dest.y - src.y) as f32;
    let mut res = y.atan2(x) * 180.0 / std::f32::consts::PI;
    res -= 270.0; // needed to set up to 0 degrees
    while res < 0.0 { // ensure angles between 0 and 360 degrees
        res += 360.0;
    }
    return res;
}

fn dist(src: &Point, dest: &Point) -> f32 {
    let dx = (dest.x - src.x) as f32;
    let dy = (dest.y - src.y) as f32;
    return (dx * dx + dy * dy).sqrt()
}

fn best_station_loc(grid: &Grid) -> (Point, i32) {
    let mut best_count = 0;
    let mut best_point = Point { x: 0, y: 0 };
    for src in grid.asteroids.iter() {
        let mut set: HashSet<i32> = HashSet::new();
        for dest in grid.asteroids.iter() {
            if src != dest { 
                // we cant use f32 as a key but need the precision
                // so include first two dec places in int
                let a = (angle(src, dest) * 100.0) as i32;
                set.insert(a);
            }
        }
        if set.len() > best_count {
            best_count = set.len();
            best_point = *src;
        }
    }
    return (best_point, best_count as i32);
}

fn laser_order(grid: &Grid, src: &Point) -> Vec<Point> {
    // BTreeMap will order our lines by angle
    let mut angles: BTreeMap<i32, Vec<Point>> = BTreeMap::new();
    for dest in grid.asteroids.iter() {
        if dest != src {
            // we cant use f32 as a key but need the precision
            // so include first two dec places in int
            let a = (angle(src, dest) * 100.0) as i32;
            angles.entry(a).or_insert(Vec::new()).push(*dest);
        }
    }
    // sort every vec in the BTreeMap by distance to the src
    for (_angle, dests) in angles.iter_mut() {
        dests.sort_unstable_by_key(|dest| (dist(src, &dest) * 100.0) as i32);
    }

    // pop the closest asteroid on each line
    let mut order: Vec<Point> = Vec::new();
    for _ in 0..grid.asteroids.len() - 1 {
        for (_angle, line) in angles.iter_mut() {
            if line.len() > 0 {
                order.push(line.remove(0));
            }
        }
    }

    return order;
}

fn main() {
    let grid = Grid::new("input.txt");

    let (best_loc, los_count) = best_station_loc(&grid);
    println!("Best LOS Count: {} at ({},{})", los_count, best_loc.x, best_loc.y);

    let order = laser_order(&grid, &best_loc);

    println!("The 200th asteroid to be vaporized is at {:?}.", order[199]);
    println!("Part 2 answer: {}", order[199].x * 100 + order[199].y);
}
