use std::process::exit;
use std::sync::mpsc::{channel, Receiver};

use crate::intcode_computer::IntcodeComputer;

pub fn process(input: Vec<&str>) {
    let tmp = input.join("");
    let numbers: Vec<&str> = tmp.split(',').collect();
    let num_vec: Vec<i32> = numbers.iter().map(|x| -> i32 { x.parse().expect("") }).collect();

    let mut highest = 0;
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    'outer: for e in 0..5 {
                        let mut arr = vec![0; 5];
                        arr[a as usize] += 1;
                        arr[b as usize] += 1;
                        arr[c as usize] += 1;
                        arr[d as usize] += 1;
                        arr[e as usize] += 1;
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
                        IntcodeComputer::new(num_vec.as_slice(), recv_a, send_a, false).run();
                        IntcodeComputer::new(num_vec.as_slice(), recv_b, send_b, false).run();
                        IntcodeComputer::new(num_vec.as_slice(), recv_c, send_c, false).run();
                        IntcodeComputer::new(num_vec.as_slice(), recv_d, send_d, false).run();
                        IntcodeComputer::new(num_vec.as_slice(), recv_e, send_e, false).run();
                        let result = recv_h.recv().unwrap();
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

fn print_channel(recv: &Receiver<i32>) {
    loop {
        if let Ok(data) = recv.recv() {
            println!("Output: {}", data);
        } else {
            break;
        }
    }
}
