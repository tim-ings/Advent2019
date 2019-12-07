use std::fs;

fn load_tape(file_path: &str) -> Vec<i32> {
    // read our input file
    let line = fs::read_to_string(file_path).unwrap();
    let line = line.trim();

    // convert the line to a vector of ints
    let mut tape: Vec<i32> = Vec::new();
    for s in line.split(",") {
        let i = s.parse::<i32>().unwrap();
        tape.push(i);
    }
    return tape;
}

fn run_tape(tape: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
    tape[1] = noun;
    tape[2] = verb;
    let mut i: usize = 0; // current tape index
    while i < tape.len() {
        let val = tape[i];
        match val {
            1 => { // add
                // get addresses
                let addr0: usize = tape[i + 1] as usize;
                let addr1: usize = tape[i + 2] as usize;
                let addr_res: usize = tape[i + 3] as usize;
                // update tape
                tape[addr_res] = tape[addr0] + tape[addr1];
            },
            2 => { // multiply
                // get addresses
                let addr0: usize = tape[i + 1] as usize;
                let addr1: usize = tape[i + 2] as usize;
                let addr_res: usize = tape[i + 3] as usize;
                // update tape
                tape[addr_res] = tape[addr0] * tape[addr1];
            },
            99 => { // halt
                return tape[0];
            },
            _ => (),
        }
        i += 4; // skip forward to the next opcode
    }
    return tape[0];
}

#[allow(dead_code)]
fn print_tape(tape: &Vec<i32>) {
    for i in tape.iter() {
        print!("{},", i);
    }
    println!();
}

fn main() {
    // get the length of the tape so we dont overflow our vector
    let tape_len: i32 = load_tape("input.txt").len() as i32;
    // loop over all combinations of noun and verb
    'outer: for noun in 0..tape_len {
        for verb in 0..tape_len {
            // load a fresh tape
            let mut tape: Vec<i32> = load_tape("input.txt");
            // calculate the result with the given noun and verb
            let res = run_tape(&mut tape, noun, verb);
            // check if we should stop
            if res == 19690720 {
                println!("Noun = {}; Verb = {}; 100 * noun + verb = {}", noun, verb, 100 * noun + verb);
                break 'outer; // loop label
            }
        }
    }
}
