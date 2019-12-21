use std::process::exit;

use fixed_map::Map;

pub fn process(input: Vec<&str>) {
    let instructions = fill_instructions();
    let numbers: Vec<&str> = input.first().expect("").split(',').collect();
    let mut num_vec: Vec<i32> = numbers.iter().map(|x| -> i32 { x.parse().expect("") }).collect();
    let numbers: &mut [i32] = num_vec.as_mut();
    let mut state = State {
        param_modes: [false, false, false],
        ip: 0,
    };

    loop {
        state.param_modes = decode_params(numbers[state.ip]);
        if let Some(ins) = instructions.get(decode_instruction(numbers[state.ip])) {
            println!("{}: {}", state.ip, ins.name);
            (ins.exec)(&mut state, numbers);
            state.ip += ins.ip_change;
        } else {
            println!("{}: NO INSTRUCTION, instead {}", state.ip, numbers[state.ip]);
            exit(1);
        }
    }
}

fn get<'a>(state: &State, mem: &'a mut [i32], pos: usize) -> &'a mut i32 {
    if state.param_modes[pos - 1] {
        mem.get_mut(state.ip + pos).unwrap()
    } else {
        mem.get_mut(mem[state.ip + pos] as usize).unwrap()
    }
}

fn decode_params(value: i32) -> [bool; 3] {
    let string = value.to_string();
    let mut chars = string.chars();
    [chars.nth_back(2).unwrap_or('0') == '1', chars.next_back().unwrap_or('0') == '1', chars.next_back().unwrap_or('0') == '1']
}

fn decode_instruction(value: i32) -> usize {
    (value % 100) as usize
}

struct State {
    param_modes: [bool; 3],
    ip: usize,
}

struct Instruction {
    name: &'static str,
    ip_change: usize,
    exec: fn(state: &mut State, mem: &mut [i32]),
}

fn fill_instructions() -> Map<usize, Instruction> {
    let mut map = Map::new();
    map.insert(99, Instruction {
        name: "HALT",
        ip_change: 1,
        exec: |_: &mut State, _: &mut [i32]| {
            println!("HALTED");
            exit(0);
        },
    });
    map.insert(1, Instruction {
        name: "ADD",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i32]| {
            *get(state, mem, 3) = *get(state, mem, 1) + *get(state, mem, 2);
        },
    });
    map.insert(2, Instruction {
        name: "MUL",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i32]| {
            *get(state, mem, 3) = *get(state, mem, 1) * *get(state, mem, 2);
        },
    });
    map.insert(3, Instruction {
        name: "IN",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut [i32]| {
            use std::io::{stdin, stdout, Write};
            let mut s = String::new();
            print!("Input: ");
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }

            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            println!("You typed: {}", s);
            *get(state, mem, 1) = s.parse().unwrap();
        },
    });
    map.insert(4, Instruction {
        name: "OUT",
        ip_change: 2,
        exec: |state: &mut State, mem: &mut [i32]| {
            println!("Output: {}", *get(state, mem, 1));
        },
    });
    map.insert(5, Instruction {
        name: "JNZ",
        ip_change: 3,
        exec: |state: &mut State, mem: &mut [i32]| {
            if *get(state, mem, 1) != 0 {
                state.ip = *get(state, mem, 2) as usize - 3;
            }
        },
    });
    map.insert(6, Instruction {
        name: "JZ",
        ip_change: 3,
        exec: |state: &mut State, mem: &mut [i32]| {
            if *get(state, mem, 1) == 0 {
                state.ip = *get(state, mem, 2) as usize - 3;
            }
        },
    });
    map.insert(7, Instruction {
        name: "LESS",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i32]| {
            *get(state, mem, 3) = if *get(state, mem, 1) < *get(state, mem, 2) {
                1
            } else {
                0
            };
        },
    });
    map.insert(8, Instruction {
        name: "EQUALS",
        ip_change: 4,
        exec: |state: &mut State, mem: &mut [i32]| {
            *get(state, mem, 3) = if *get(state, mem, 1) == *get(state, mem, 2) {
                1
            } else {
                0
            };
        },
    });
    map
}

