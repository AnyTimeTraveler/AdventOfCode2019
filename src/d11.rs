use std::sync::mpsc::channel;
use std::thread;

use crate::intcode_computer::IntcodeComputer;

pub fn process(input: String) {
    let string = input.replacen("\n", "", 100_000);
    let string = string.replacen(" ", "", 100_000);
    let numbers: Vec<&str> = string.split(',').collect();
    let num_vec: Vec<i64> = numbers.iter().map(|x| -> i64 { x.parse().expect("") }).collect();


    let (send_p, recv_c) = channel();
    let (send_c, recv_p) = channel();

    let comp_handle = thread::spawn(move || {
        let mut comp = IntcodeComputer::new(num_vec.as_slice(), recv_c, send_c, false);
//        comp.add_watch(100);
//        comp.add_watch(101);
        comp.run();
    });

    let paint_handle = thread::spawn(move || {
        let mut map_touched = [[false; 100]; 100];
        let mut map_colour = [[false; 100]; 100];
        let mut pos: (usize, usize, i8) = (50, 50, 0);
        map_colour[pos.0][pos.1] = true;
        loop {
            //send color
            let _ = send_p.send(if map_colour[pos.0][pos.1] { 1 } else { 0 });
            match recv_p.recv() {
                Ok(color) => {
                    map_colour[pos.0][pos.1] = color == 1;
                    map_touched[pos.0][pos.1] = true;
                }
                Err(_) => {
                    break;
                }
            }
            match recv_p.recv() {
                Ok(direction) => {
                    if direction == 0 { pos.2 -= 1; } else { pos.2 += 1; };
                    pos.2 %= 4;
                    if pos.2 < 0 {
                        pos.2 += 4;
                    }
                }
                Err(_) => {
                    break;
                }
            }
            println!("Dir: {}", pos.2);
            match pos.2 {
                0 => // north
                    pos.0 -= 1,

                1 => // east
                    pos.1 += 1,

                2 => // south
                    pos.0 += 1,

                3 => // west
                    pos.1 -= 1,

                _ => panic!("Logic error"),
            }
        }
        (map_touched, map_colour)
    });
    let _ = comp_handle.join();
    if let Ok((touched, map)) = paint_handle.join() {
        let count = touched.iter()
            .flat_map(|row| { row.iter() })
            .filter(|a| { **a })
            .count();
        println!("{}", count);

        for row in map.iter() {
            for pixel in row.iter() {
                print!("{}", if *pixel { 'X' } else { ' ' });
            }
            println!();
        }
    } else {
        println!("Nop!");
    }
}
