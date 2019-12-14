extern crate queues;

use std::fs;
use queues::*;


enum Opcode {
    ADD,
    MUL,
    INP,
    OUT,
    JIT,
    JIF,
    LT,
    EQ,
    HALT,
}

impl Opcode {
    fn from_i32(i: i32) -> Self {
        match i {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::INP,
            4 => Opcode::OUT,
            5 => Opcode::JIT,
            6 => Opcode::JIF,
            7 => Opcode::LT,
            8 => Opcode::EQ,
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
            Opcode::ADD | Opcode::MUL | Opcode::LT | Opcode::EQ => {
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
            Opcode::JIT | Opcode::JIF => {
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
            let mut jumped = false;
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
                Opcode::JIT => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let idx = inst.parameters[1].get_value(&self.memory);
                    if val0 != 0 {
                        i = idx as usize;
                        jumped = true;
                    }
                },
                Opcode::JIF => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let idx = inst.parameters[1].get_value(&self.memory);
                    if val0 == 0 {
                        i = idx as usize;
                        jumped = true;
                    }
                },
                Opcode::LT => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let val1 = inst.parameters[1].get_value(&self.memory);
                    let idx = inst.parameters[2].get_value(&self.memory) as usize;
                    if val0 < val1 {
                        self.memory[idx] = 1;
                    } else {
                        self.memory[idx] = 0;
                    }
                },
                Opcode::EQ => {
                    let val0 = inst.parameters[0].get_value(&self.memory);
                    let val1 = inst.parameters[1].get_value(&self.memory);
                    let idx = inst.parameters[2].get_value(&self.memory) as usize;
                    if val0 == val1 {
                        self.memory[idx] = 1;
                    } else {
                        self.memory[idx] = 0;
                    }
                },
                Opcode::HALT => return,
            }
            if !jumped {
                i += inst.len();
            }
        }
    }
}

fn run_amplifiers(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i32 {
    let phase_settings = vec![p0, p1, p2, p3, p4];
    let mut last_output = 0;
    for i in 0..5 {
        let mut cpu = Computer::new("input.txt");
        cpu.input.add(phase_settings[i]).expect("failed to add to queue");
        cpu.input.add(last_output).expect("failed to add to queue");
        cpu.run();
        last_output = *cpu.output.first().expect("cpu should have outputted a value");
    }
    return last_output;
}

fn main() {
    let mut max_thrust = 0;
    // not pretty
    for p0 in 0..5 {
        for p1 in 0..5 {
            if p1 == p0 { continue; } // ignore combinations
            for p2 in 0..5 {
                if p2 == p0 || p2 == p1 { continue; } // ignore combinations
                for p3 in 0..5 {
                    if p3 == p0 || p3 == p1 || p3 == p2 { continue; } // ignore combinations
                    for p4 in 0..5 {
                        if p4 == p0 || p4 == p1 || p4 == p2 || p4 == p3 { continue; } // ignore combinations
                        let new_thrust = run_amplifiers(p0, p1, p2, p3, p4);
                        if new_thrust > max_thrust {
                            max_thrust = new_thrust;
                            println!("Found new max thrust of {} at {}, {}, {}, {}, {}", max_thrust, p0, p1, p2, p3, p4);
                        }
                    }
                }
            }
        }
    }
    println!("Max Thrust: {}", max_thrust);
}
