use std::sync::mpsc::channel;
use super::intcode_computer::IntcodeComputer;

pub fn process(input: Vec<&str>) {
    let numbers: Vec<&str> = input.first().expect("").split(',').collect();
    let num_vec: Vec<i32> = numbers.iter().map(|x| -> i32 { x.parse().expect("") }).collect();
    let (in_sender, in_receiver) = channel();
    let (out_sender, out_receiver) = channel();
    let _ = in_sender.send(1);
    IntcodeComputer::new(num_vec.as_slice(), in_receiver, out_sender, true).run();
    loop {
        if let Ok(data) = out_receiver.recv() {
            println!("Output: {}", data);
        } else {
            break;
        }
    }
}
