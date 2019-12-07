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

fn run_tape(tape: &mut Vec<i32>) {
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
                return
            },
            _ => (),
        }
        i += 4; // skip forward to the next opcode
    }
    return
}

fn print_tape(tape: &Vec<i32>) {
    for i in tape.iter() {
        print!("{},", i);
    }
    println!();
}

fn main() {
    let mut tape: Vec<i32> = load_tape("input.txt");
    tape[1] = 12;
    tape[2] = 2;
    run_tape(&mut tape);
    print_tape(&tape);
    println!("First Value: {}", tape[0]);
}
