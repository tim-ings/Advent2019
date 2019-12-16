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
    ARB,
    HALT,
}

impl Opcode {
    fn from_i64(i: i64) -> Self {
        match i {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::INP,
            4 => Opcode::OUT,
            5 => Opcode::JIT,
            6 => Opcode::JIF,
            7 => Opcode::LT,
            8 => Opcode::EQ,
            9 => Opcode::ARB,
            99 => Opcode::HALT,
            _ => panic!("Unknown opcode {}", i),
        }
    }

    fn param_count(&self) -> u32 {
        match self {
            Opcode::ADD => 3,
            Opcode::MUL => 3,
            Opcode::INP => 1,
            Opcode::OUT => 1,
            Opcode::JIT => 2,
            Opcode::JIF => 2,
            Opcode::LT => 3,
            Opcode::EQ => 3,
            Opcode::ARB => 1,
            Opcode::HALT => 0,
        }
    }
}

enum ParamMode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

impl ParamMode {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => ParamMode::POSITION,
            1 => ParamMode::IMMEDIATE,
            2 => ParamMode::RELATIVE,
            _ => panic!("Unknown opcode {}", i),
        }
    }
}

struct Parameter {
    mode: ParamMode,
    value: i64,
}

impl Parameter {
    fn get_value(&self, comp: &mut Computer) -> i64 {
        match self.mode {
            ParamMode::POSITION => comp.read(self.value as usize),
            ParamMode::IMMEDIATE => self.value,
            ParamMode::RELATIVE => comp.read((self.value + (comp.relative_base as i64)) as usize),
        }
    }

    fn get_idx(&self, comp: &mut Computer) -> usize {
        match self.mode {
            ParamMode::POSITION => self.value as usize,
            ParamMode::IMMEDIATE => panic!("Index should never be in immidiate mode"),
            ParamMode::RELATIVE => (self.value + (comp.relative_base as i64)) as usize,
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

    fn new(icode: i64, comp: &mut Computer) -> Self {
        // parse the opcode from the instruction code
        let opcode = Opcode::from_i64(icode % 100);
        // get the param modes and values for each param in the instruction
        let param_count = opcode.param_count();
        let mut params = Vec::new();
        for i in 0..param_count {
            params.push(Parameter {
                mode: ParamMode::from_i64((icode / (100 * 10i64.pow(i))) % 10),
                value: comp.read(comp.inst_pointer + 1 + i as usize),
            });
        }
        return Instruction {
            opcode: opcode,
            parameters: params,
        };
    }
}

struct Computer {
    memory: Vec<i64>,
    input: Queue<i64>,
    output: Queue<i64>,
    inst_pointer: usize,
    relative_base: usize,
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
                    .map(|x| x.parse::<i64>().expect("Unable to parse"))
                    .collect(),
            input: Queue::new(),
            output: Queue::new(),
            inst_pointer: 0,
            relative_base: 0,
        }
    }

    fn read(&mut self, addr: usize) -> i64 {
        if self.memory.len() < addr {
            self.memory.resize(addr + 10, 0);
        }
        return self.memory[addr];
    }

    fn write(&mut self, addr: usize, value: i64) {
        if self.memory.len() <= addr {
            self.memory.resize(addr + 10, 0);
        }
        self.memory[addr] = value;
    }

    fn run(&mut self) -> bool {
        while self.inst_pointer < self.memory.len() {
            let icode = self.memory[self.inst_pointer];
            let inst = Instruction::new(icode, self);
            let mut jumped = false;
            match inst.opcode {
                Opcode::ADD => {
                    let lhs = inst.parameters[0].get_value(self);
                    let rhs = inst.parameters[1].get_value(self);
                    let idx = inst.parameters[2].get_idx(self);
                    self.write(idx, lhs + rhs);
                },
                Opcode::MUL => {
                    let lhs = inst.parameters[0].get_value(self);
                    let rhs = inst.parameters[1].get_value(self);
                    let idx = inst.parameters[2].get_idx(self);
                    self.write(idx, lhs * rhs);
                },
                Opcode::INP => {
                    let idx = inst.parameters[0].get_idx(self);
                    if self.input.size() < 1 {
                        return false; // we have not halted but are waiting on input
                    }
                    let inp = self.input.remove().unwrap();
                    self.write(idx, inp);
                },
                Opcode::OUT => {
                    match inst.parameters[0].mode {
                        ParamMode::IMMEDIATE => {
                            self.output.add(inst.parameters[0].value).unwrap();
                        },
                        ParamMode::POSITION => {
                            let outp = self.read(inst.parameters[0].value as usize);
                            self.output.add(outp).unwrap();
                        },
                        ParamMode::RELATIVE => {
                            let idx = (self.relative_base as i64) + inst.parameters[0].value;
                            let outp = self.read(idx as usize);
                            self.output.add(outp).unwrap();
                        }
                    };
                },
                Opcode::JIT => {
                    let test = inst.parameters[0].get_value(self);
                    let new_ip = inst.parameters[1].get_value(self) as usize;
                    if test != 0 {
                        self.inst_pointer = new_ip;
                        jumped = true; // make sure we dont increment the instruction pointer after the jump
                    }
                },
                Opcode::JIF => {
                    let test = inst.parameters[0].get_value(self);
                    let new_ip = inst.parameters[1].get_value(self) as usize;
                    if test == 0 {
                        self.inst_pointer = new_ip;
                        jumped = true; // make sure we dont increment the instruction pointer after the jump
                    }
                },
                Opcode::LT => {
                    let p0 = inst.parameters[0].get_value(self);
                    let p1 = inst.parameters[1].get_value(self);
                    let idx = inst.parameters[2].get_idx(self);
                    if p0 < p1 {
                        self.write(idx, 1);
                    } else {
                        self.write(idx, 0);
                    }
                },
                Opcode::EQ => {
                    let p0 = inst.parameters[0].get_value(self);
                    let p1 = inst.parameters[1].get_value(self);
                    let idx = inst.parameters[2].get_idx(self);
                    if p0 == p1 {
                        self.write(idx, 1);
                    } else {
                        self.write(idx, 0);
                    }
                },
                Opcode::ARB =>  {
                    let p0 = inst.parameters[0].get_value(self);
                    let rb = self.relative_base as i64;
                    if rb < 0 {
                        panic!("Relative base should not be negative");
                    }
                    self.relative_base = (rb + p0) as usize;
                },
                Opcode::HALT => {
                    return true;
                },
            };
            // if we didnt jump we increment the instruction pointer by the len of the instruction
            if !jumped {
                self.inst_pointer += inst.len();
            }
        }
        return true; // we have halted
    }
}

fn main() {
    let mut comp = Computer::new("input.txt");
    comp.input.add(2).unwrap();
    comp.run();
    while comp.output.size() > 0 {
        println!("{}", comp.output.remove().unwrap());
    }
}