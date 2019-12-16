use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    asteroid: bool,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Point>,
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

        let width = lines[0].len();
        let height = lines.len();

        let mut data = Vec::new();
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                data.push(Point {
                    x: x as i32, 
                    y: y as i32,
                    asteroid: match ch {
                        '.' => false,
                        '#' => true,
                        _ => panic!(format!("Bad symbol in input: {}", ch)),
                    },
                });
            }
        };

        return Grid {
            width: width,
            height: height,
            data: data,
        };
    }
}

fn dist(p0: &Point, p1: &Point) -> f32 {
    let dy = (p1.y - p0.y) as f32;
    let dx = (p1.x - p0.x) as f32;
    return (dx * dx + dy * dy).sqrt();
}

fn is_blocking(src: &Point, dest: &Point, blocker: &Point) -> bool {
    let d0 = dist(src, blocker) + dist(blocker, dest);
    let d1 = dist(src, dest);
    return (d0 - d1).abs() < 0.0001;
}

fn fmt_i32(i: i32) -> String {
    match i {
        0..=9 => format!("    {}", i),
        10..=99 => format!("   {}", i),
        100..=999 => format!("  {}", i),
        1000..=9999 => format!(" {}", i),
        10000..=99999 => format!("{}", i),
        _ => i.to_string(),
    }
}

fn main() {
    let grid = Grid::new("input.txt");

    let mut los_count: Vec<i32> = vec![0; grid.width * grid.height];

    // check every src -> dest combination
    for src in grid.data.iter() {
        if !src.asteroid { continue; } // only check points with asteroids

        for dest in grid.data.iter() {
            if !dest.asteroid { continue; } // only check points with asteroids

            // check if any asteroids are blocking src -> dest
            let mut in_los = true;
            for blocker in grid.data.iter() {
                // only check blockers with astreoids
                // and that are not the src or the dest 
                if !blocker.asteroid || src == blocker || dest == blocker { continue; }

                if is_blocking(src, dest, blocker) {
                    in_los = false;
                    break;
                }
            }
            if in_los {
                los_count[(src.y * (grid.width as i32) + src.x) as usize] += 1;
            }
        }
    }

    // find the best asteroid
    let mut best_los_count = 0;
    let mut best_los_x = 0;
    let mut best_los_y = 0;
    let should_print = true;

    for i in 0..grid.height {
        for j in 0..grid.width {
            let lc = los_count[i * grid.height + j];
            if lc > best_los_count {
                best_los_count = lc - 1;
                best_los_x = j;
                best_los_y = i;
            }
            if should_print {
                if lc != 0 {
                    print!("{}", fmt_i32(lc - 1));
                } else {
                    print!("    .");
                }
            }
        }
        if should_print {
            println!("");
        }
    }

    println!("Best LOS Count: {} at ({},{})", best_los_count, best_los_x, best_los_y);
}
