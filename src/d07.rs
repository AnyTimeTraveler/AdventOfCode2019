use std::sync::mpsc::channel;
use std::thread;

use crate::intcode_computer::IntcodeComputer;

pub fn process(input: Vec<&str>) {
    let tmp = input.join("");
    let numbers: Vec<&str> = tmp.split(',').collect();
    let num_vec: Vec<i64> = numbers.iter().map(|x| -> i64 { x.parse().expect("") }).collect();

    let mut highest = 0;
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    'outer: for e in 5..=9 {
                        // deduplicate
                        let mut arr = vec![0; 5];
                        arr[a as usize - 5] += 1;
                        arr[b as usize - 5] += 1;
                        arr[c as usize - 5] += 1;
                        arr[d as usize - 5] += 1;
                        arr[e as usize - 5] += 1;
                        for number in arr {
                            if number > 1 {
                                continue 'outer;
                            }
                        }

                        let (send_h, recv_a) = channel();
                        let (send_a, recv_b) = channel();
                        let (send_b, recv_c) = channel();
                        let (send_c, recv_d) = channel();
                        let (send_d, recv_e) = channel();
                        let (send_e, recv_h) = channel();
                        let _ = send_h.send(a); // phase
                        let _ = send_a.send(b); // phase
                        let _ = send_b.send(c); // phase
                        let _ = send_c.send(d); // phase
                        let _ = send_d.send(e); // phase
                        let _ = send_h.send(0); // init input

                        let mut computer_a = IntcodeComputer::new(num_vec.as_slice(), recv_a, send_a, false);
                        let mut computer_b = IntcodeComputer::new(num_vec.as_slice(), recv_b, send_b, false);
                        let mut computer_c = IntcodeComputer::new(num_vec.as_slice(), recv_c, send_c, false);
                        let mut computer_d = IntcodeComputer::new(num_vec.as_slice(), recv_d, send_d, false);
                        let mut computer_e = IntcodeComputer::new(num_vec.as_slice(), recv_e, send_e, false);
                        thread::spawn(move || { computer_a.run() });
                        thread::spawn(move || { computer_b.run() });
                        thread::spawn(move || { computer_c.run() });
                        thread::spawn(move || { computer_d.run() });
                        let handle = thread::spawn(move || { computer_e.run() });
                        let mut result = 0;
                        loop {
                            if let Ok(data) = recv_h.recv() {
                                result = data;
//                                println!("Output: {}", data);
                                let _ = send_h.send(data);
                            } else {
                                break;
                            }
                        }
                        let _ = handle.join();
                        if result > highest {
                            println!("{} -> {}", highest, result);
                            println!("{} {} {} {} {}", a, b, c, d, e);
                            highest = result;
                        }
                    }
                }
            }
        }
    }
    println!("Result: {}", highest);
}
