extern crate termion;

use std::{io, thread};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::net::TcpStream;
use std::process::exit;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use termion::raw::IntoRawMode;

use crate::intcode_computer::IntcodeComputer;

use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::screen::AlternateScreen;

pub fn process(input: String) {
    let string = input.replacen("\n", "", 100_000);
    let string = string.replacen(" ", "", 100_000);
    let numbers: Vec<&str> = string.split(',').collect();
    let mut num_vec: Vec<i64> = numbers.iter().map(|x| -> i64 { x.parse().expect("") }).collect();

    num_vec[0] = 2; // free play

    let (send_h, recv_c) = channel();
    let (send_c, recv_h) = channel();
    let (send_log, recv_log) = channel();

    let comp_handle = thread::spawn(move || IntcodeComputer::new(num_vec.as_slice(), recv_c, send_c, Some(send_log)).run());


    let log_handle: JoinHandle<io::Result<()>> = thread::spawn(move || handle_logs(recv_log));

    let display_handle = thread::spawn(move || handle_display(recv_h));

    let input_handle = thread::spawn(move || handle_input(send_h));

    comp_handle.join().unwrap();
    println!("Waiting for display handle...");
    if let Ok(Ok(map)) = display_handle.join() {
        println!("Tiles: {}", map.iter().filter(|((_, _), t)| { **t == 2 }).count());
    }

    println!("Waiting for log handle...");
    log_handle.join().unwrap().unwrap();
    println!("Waiting for input handle...");
    input_handle.join().unwrap();
}

fn handle_logs(recv: Receiver<String>) -> io::Result<()> {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:1337")?;
    stream.set_nodelay(true)?;
    loop {
        if let Ok(log) = recv.recv() {
            stream.write(log.as_bytes())?;
            stream.flush()?;
        }
    }
}

fn handle_input(send: Sender<i64>) -> () {
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Left => { let _ = send.send(-1); }
            Key::Right => { let _ = send.send(1); }
            Key::Char('q') => { exit(0); }
            Key::Ctrl('c') => { exit(0); }
            _ => { let _ = send.send(0); }
        }
    }
}

fn handle_display(recv: Receiver<i64>) -> Result<HashMap<(i64, i64), i64>, io::Error> {
    let mut objects = HashMap::new();
    objects.insert(0i64, ' ');
    objects.insert(1, '#');
    objects.insert(2, 'x');
    objects.insert(3, '=');
    objects.insert(4, 'o');
    let objects = objects;

    let mut screen = AlternateScreen::from(stdout().into_raw_mode()?);
    let mut map = HashMap::new();

    while let Ok(x) = recv.recv() {
        let y = recv.recv().unwrap();
        let tile = recv.recv().unwrap();
        if x == -1 && y == 0 {
            write!(screen, "{}{}", termion::cursor::Goto(4, 28), tile)?;
        } else {
            map.insert((x, y), tile);
            write!(screen, "{}{}{}",
                   termion::cursor::Goto(x as u16 + 1, y as u16 + 1),
                   objects.get(&tile).unwrap(),
                   termion::cursor::Goto(4, 28))?;
        }
        screen.flush().unwrap();
    }
    write!(screen, "{}", termion::cursor::Show).unwrap();
    Ok(map)
}
