extern crate queues;

use std::fs;
use queues::*;


enum Opcode {
    ADD,
    MUL,
    INP,
    OUT,
    HALT,
}

impl Opcode {
    fn from_i32(i: i32) -> Self {
        match i {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::INP,
            4 => Opcode::OUT,
            99 => Opcode::HALT,
            _ => panic!("Unknown opcode {}", i),
        }
    }
}

enum ParamMode {
    POSITION,
    IMMEDIATE,
}

impl ParamMode {
    fn from_i32(i: i32) -> Self {
        match i {
            0 => ParamMode::POSITION,
            1 => ParamMode::IMMEDIATE,
            _ => panic!("Unknown opcode {}", i),
        }
    }
}

struct Parameter {
    mode: ParamMode,
    value: i32,
}

impl Parameter {
    fn get_value(&self, memory: &Vec<i32>) -> i32 {
        match self.mode {
            ParamMode::POSITION => memory[self.value as usize],
            ParamMode::IMMEDIATE => self.value,
        }
    }
}

struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

impl Instruction {
    fn len(&self) -> usize {
        return self.parameters.len() + 1;
    }

    fn new(icode: i32, memory: &Vec<i32>, index: usize) -> Self {
        let opcode = Opcode::from_i32(icode % 100);
        match opcode {
            Opcode::ADD | Opcode::MUL => {
                Instruction {
                    opcode: opcode,
                    parameters: vec![
                        Parameter {
                            mode: ParamMode::from_i32((icode / 100) % 10),
                            value: memory[index + 1],
                        },
                        Parameter {
                            mode: ParamMode::from_i32((icode / 1000) % 10),
                            value: memory[index + 2],
                        },
                        Parameter {
                            mode: ParamMode::IMMEDIATE,
                            value: memory[index + 3],
                        },
                    ],
                }
            },
            Opcode::INP | Opcode::OUT => {
                Instruction {
                    opcode: opcode,
                    parameters: vec![
                        Parameter {
                            mode: ParamMode::IMMEDIATE,
                            value: memory[index + 1],
                        },
                    ],
                }
            },
            Opcode::HALT => {
                Instruction {
                    opcode: opcode,
                    parameters: Vec::new(),
                }
            },
        }
    }
}

struct Computer {
    memory: Vec<i32>,
    input: Queue<i32>,
    output: Vec<i32>,
}

impl Computer {
    fn new(file_path: &str) -> Self {
        Computer {
            memory: fs::read_to_string(file_path)
                    .expect("Unable to read file")
                    .lines()
                    .next()
                    .expect("Invalid input")
                    .split(",")
                    .map(|x| x.parse::<i32>().expect("Unable to parse"))
                    .collect(),
            input: Queue::new(),
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        let mut i: usize = 0;
        while i < self.memory.len() {
            let icode = self.memory[i];
            let inst = Instruction::new(icode, &self.memory, i);
            match inst.opcode {
                Opcode::ADD => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let val1 = inst.parameters[1].get_value(&self.memory);
                    let idx = inst.parameters[2].get_value(&self.memory) as usize;
                    self.memory[idx] = val0 + val1;
                },
                Opcode::MUL => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let val1 = inst.parameters[1].get_value(&self.memory);
                    let idx = inst.parameters[2].get_value(&self.memory) as usize;
                    self.memory[idx] = val0 * val1;
                },
                Opcode::INP => {
                    let idx = inst.parameters[0].get_value(&self.memory) as usize;
                    self.memory[idx] = self.input.remove().expect("Input queue is empty");
                },
                Opcode::OUT => {
                    let idx = inst.parameters[0].get_value(&self.memory) as usize;
                    self.output.push(self.memory[idx]);
                },
                Opcode::HALT => return,
            }
            i += inst.len();
        }
    }
}

fn main() {
    let mut comp = Computer::new("input.txt");
    comp.input.add(1).expect("failed to add to queue");
    comp.run();
    for o in comp.output {
        println!("{}", o);
    }
}
