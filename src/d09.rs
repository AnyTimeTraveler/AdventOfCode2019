use std::sync::mpsc::channel;

use crate::intcode_computer::IntcodeComputer;

pub fn process(input: String) {
    let string = input.replacen("\n", "", 100_000);
    let string = string.replacen(" ", "", 100_000);
    let numbers: Vec<&str> = string.split(',').collect();
    let num_vec: Vec<i64> = numbers.iter().map(|x| -> i64 { x.parse().expect("") }).collect();

    let (send_h, recv_c) = channel();
    let (send_c, recv_h) = channel();
    let _ = send_h.send(1);
    {
        let mut comp = IntcodeComputer::new(num_vec.as_slice(), recv_c, send_c, false);
//        comp.add_watch(100);
//        comp.add_watch(101);
        comp.run();
    }
    while let Ok(data) = recv_h.recv() {
        println!("Output: {}", data);
    }
}
