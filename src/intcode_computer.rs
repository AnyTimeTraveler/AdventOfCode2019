use std::process::exit;
use std::sync::mpsc::{Receiver, Sender};

use fixed_map::Map;

pub struct IntcodeComputer {
    instructions: Map<usize, Instruction>,
    memory: Vec<i64>,
    state: State,
    watches: Vec<Watch>,
}

#[derive(Debug, Copy, Clone)]
enum AddressMode {
    IMMEDIATE,
    ABSOLUTE,
    RELATIVE,
}

struct State {
    param_modes: [AddressMode; 3],
    relative_base: usize,
    ip: usize,
    halt: bool,
    input: Receiver<i64>,
    output: Sender<i64>,
    debug_output: Option<Sender<String>>,
}

struct Instruction {
    name: &'static str,
    ip_change: usize,
    exec: fn(state: &mut State, mem: &mut Vec<i64>),
}

struct Watch {
    address: usize,
    last_value: Option<i64>,
}

impl IntcodeComputer {
    pub fn new(program: &[i64], input: Receiver<i64>, output: Sender<i64>, debug_output: Option<Sender<String>>) -> IntcodeComputer {
        let mut memory = vec![0; program.len()];
        memory.copy_from_slice(&program[..]);

        IntcodeComputer {
            instructions: fill_instructions(),
            memory,
            state: State {
                param_modes: [AddressMode::IMMEDIATE; 3],
                relative_base: 0,
                ip: 0,
                halt: false,
                input,
                output,
                debug_output,
            },
            watches: Vec::new(),
        }
    }
    pub fn step(&mut self) {
        self.state.param_modes = decode_params(self.memory[self.state.ip]);
        if let Some(ins) = self.instructions.get(decode_instruction(self.memory[self.state.ip])) {
            if let Some(log) = &self.state.debug_output {
                let _ = log.send(format!("{}: {}\n", self.state.ip, ins.name));
            }
            (ins.exec)(&mut self.state, &mut self.memory);
            self.state.ip += ins.ip_change;
        } else {
            eprintln!("{}: NO INSTRUCTION, instead {}", self.state.ip, self.memory[self.state.ip]);
            exit(1);
        }
    }
    pub fn run(&mut self) {
        while !self.state.halt {
            self.step();
            self.check_watches();
        }
    }
    pub fn add_watch(&mut self, address: usize) {
        self.watches.push(Watch {
            address,
            last_value: None,
        })
    }
    fn check_watches(&mut self) {
        for mut watch in &mut self.watches {
            let new_value = match self.memory.get(watch.address) {
                Some(a) => Some(*a),
                None => None
            };

            if watch.last_value != new_value {
                print!("Watch on {} changed:", watch.address);
                if let Some(a) = watch.last_value {
                    print!("{}", a);
                } else {
                    print!("None");
                }
                println!(" => {}", self.memory[watch.address]);
                watch.last_value = match self.memory.get(watch.address) {
                    Some(a) => Some(*a),
                    None => None
                };
            }
        }
    }
}

fn get<'a>(state: &State, mem: &'a mut Vec<i64>, pos: usize) -> &'a mut i64 {
    match &state.param_modes[pos - 1] {
        AddressMode::IMMEDIATE => mem.get_mut(state.ip + pos).unwrap(),
        AddressMode::ABSOLUTE => {
            let position = mem[state.ip + pos] as usize;
            fit_memory(mem, position, &state.debug_output);
            mem.get_mut(position).unwrap()
        }
        AddressMode::RELATIVE => {
            let mut position = mem[state.ip + pos];
            position += state.relative_base as i64;
            let position = position as usize;
            fit_memory(mem, position, &state.debug_output);
            mem.get_mut(position).unwrap()
        }
    }
}

fn fit_memory(mem: &mut Vec<i64>, position: usize, debug: &Option<Sender<String>>) {
    if mem.len() <= position {
        if let Some(debug) = &debug {
            let _ = debug.send(format!("Memory too small, extending from {} to {}\n", mem.len(), position));
        }
    }
    while mem.len() <= position {
        mem.push(0);
    }
}

fn decode_params(value: i64) -> [AddressMode; 3] {
    let string = value.to_string();
    let mut chars = string.chars();
    let matcher = |c| {
        match c {
            Some('0') => AddressMode::ABSOLUTE,
            Some('1') => AddressMode::IMMEDIATE,
            Some('2') => AddressMode::RELATIVE,
            None => AddressMode::ABSOLUTE,
            _ => panic!("Invalid address mode!"),
        }
    };
    [matcher(chars.nth_back(2)), matcher(chars.next_back()), matcher(chars.next_back())]
}

fn decode_instruction(value: i64) -> usize {
    (value % 100) as usize
}

fn fill_instructions() -> Map<usize, Instruction> {
    let mut map = Map::new();
    map.insert(99, Instruction {
        name: "HALT",
        ip_change: 1,
        exec: |state: &mut State, _: &mut Vec<i64>| {
            state.halt = true;
        },
    });
    map.insert(1, Instruction {
        name: "ADD",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            *get(&state, mem, 3) = *get(&state, mem, 1) + *get(&state, mem, 2);
        },
    });
    map.insert(2, Instruction {
        name: "MUL",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            *get(&state, mem, 3) = *get(&state, mem, 1) * *get(&state, mem, 2);
        },
    });
    map.insert(3, Instruction {
        name: "IN",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            *get(&state, mem, 1) = state.input.recv().unwrap();
        },
    });
    map.insert(4, Instruction {
        name: "OUT",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            let _ = state.output.send(*get(&state, mem, 1));
        },
    });
    map.insert(5, Instruction {
        name: "JNZ",
        ip_change: 0,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            if *get(&state, mem, 1) != 0 {
                state.ip = *get(&state, mem, 2) as usize;
            } else {
                state.ip += 3;
            }
        },
    });
    map.insert(6, Instruction {
        name: "JZ",
        ip_change: 0,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            if *get(&state, mem, 1) == 0 {
                state.ip = *get(&state, mem, 2) as usize;
            } else {
                state.ip += 3;
            }
        },
    });
    map.insert(7, Instruction {
        name: "LESS",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            *get(&state, mem, 3) = if *get(&state, mem, 1) < *get(&state, mem, 2) {
                1
            } else {
                0
            };
        },
    });
    map.insert(8, Instruction {
        name: "EQUALS",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            *get(&state, mem, 3) = if *get(&state, mem, 1) == *get(&state, mem, 2) {
                1
            } else {
                0
            };
        },
    });
    map.insert(9, Instruction {
        name: "ADJ REL_BASE",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut Vec<i64>| {
            let old_base = state.relative_base as i64;
            state.relative_base = (*get(&state, mem, 1) + old_base) as usize;
        },
    });
    map
}

