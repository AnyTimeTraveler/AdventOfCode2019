use std::sync::mpsc::{Receiver, Sender};
use fixed_map::Map;
use std::process::exit;

pub struct IntcodeComputer {
    instructions: Map<usize, Instruction>,
    memory: Vec<i64>,
    state: State,
}

impl IntcodeComputer {
    pub fn new(program: &[i64], input: Receiver<i64>, output: Sender<i64>, debug: bool) -> IntcodeComputer {
        let mut memory = vec![0; program.len()];
        memory.copy_from_slice(&program[..]);

        IntcodeComputer {
            instructions: fill_instructions(),
            memory,
            state: State {
                param_modes: [false, false, false],
                ip: 0,
                halt: false,
                input,
                output,
                debug,
            },
        }
    }
    pub fn step(&mut self) {
        let mem: &mut [i64] = self.memory.as_mut();
        self.state.param_modes = decode_params(mem[self.state.ip]);
        if let Some(ins) = self.instructions.get(decode_instruction(mem[self.state.ip])) {
            if self.state.debug {
                println!("{}: {}", self.state.ip, ins.name);
            }
            (ins.exec)(&mut self.state, mem);
            self.state.ip += ins.ip_change;
        } else {
            println!("{}: NO INSTRUCTION, instead {}", self.state.ip, mem[self.state.ip]);
            exit(1);
        }
    }
    pub fn run(&mut self) {
        while !self.state.halt {
            self.step();
        }
    }
}

fn get<'a>(state: &State, mem: &'a mut [i64], pos: usize) -> &'a mut i64 {
    if state.param_modes[pos - 1] {
        mem.get_mut(state.ip + pos).unwrap()
    } else {
        mem.get_mut(mem[state.ip + pos] as usize).unwrap()
    }
}

fn decode_params(value: i64) -> [bool; 3] {
    let string = value.to_string();
    let mut chars = string.chars();
    [chars.nth_back(2).unwrap_or('0') == '1', chars.next_back().unwrap_or('0') == '1', chars.next_back().unwrap_or('0') == '1']
}

fn decode_instruction(value: i64) -> usize {
    (value % 100) as usize
}

struct State {
    param_modes: [bool; 3],
    ip: usize,
    halt: bool,
    input: Receiver<i64>,
    output: Sender<i64>,
    debug: bool,
}

struct Instruction {
    name: &'static str,
    ip_change: usize,
    exec: fn(state: &mut State, mem: &mut [i64]),
}

fn fill_instructions() -> Map<usize, Instruction> {
    let mut map = Map::new();
    map.insert(99, Instruction {
        name: "HALT",
        ip_change: 1,
        exec: |state: &mut State, _: &mut [i64]| {
            state.halt = true;
        },
    });
    map.insert(1, Instruction {
        name: "ADD",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i64]| {
            *get(&state, mem, 3) = *get(&state, mem, 1) + *get(&state, mem, 2);
        },
    });
    map.insert(2, Instruction {
        name: "MUL",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i64]| {
            *get(&state, mem, 3) = *get(&state, mem, 1) * *get(&state, mem, 2);
        },
    });
    map.insert(3, Instruction {
        name: "IN",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut [i64]| {
            *get(&state, mem, 1) = state.input.recv().unwrap();
        },
    });
    map.insert(4, Instruction {
        name: "OUT",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut [i64]| {
            let _ = state.output.send(*get(&state, mem, 1));
        },
    });
    map.insert(5, Instruction {
        name: "JNZ",
        ip_change: 3,
        exec: |state: &mut State, mem: &mut [i64]| {
            if *get(&state, mem, 1) != 0 {
                state.ip = *get(&state, mem, 2) as usize - 3;
            }
        },
    });
    map.insert(6, Instruction {
        name: "JZ",
        ip_change: 3,
        exec: |state: &mut State, mem: &mut [i64]| {
            if *get(&state, mem, 1) == 0 {
                state.ip = *get(&state, mem, 2) as usize - 3;
            }
        },
    });
    map.insert(7, Instruction {
        name: "LESS",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i64]| {
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
        exec: |state: &mut State, mem: &mut [i64]| {
            *get(&state, mem, 3) = if *get(&state, mem, 1) == *get(&state, mem, 2) {
                1
            } else {
                0
            };
        },
    });
    map
}

