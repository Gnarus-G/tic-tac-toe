use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use tic_tac_toe::{utils::input::Play, Move};

pub fn recieve_their_play(m: Move, stream: &TcpStream) -> Option<Play> {
    println!("waiting for their play...\n");
    Play::from(&read_line(stream), m)
}

pub fn send_our_play((row, col): (usize, usize), mut destination: &TcpStream) {
    match writeln!(destination, "{} {}", row, col) {
        Err(err) => println!("{}", err),
        Ok(()) => println!("our play sent successfully!"),
    }
}

pub fn recieve_board_size(stream: &TcpStream) -> usize {
    println!("waiting for them to choose a board size... \n");
    return read_line(stream)
        .parse()
        .expect("Couldn't parse board size from the input recieved.");
}

pub fn send_board_size(size: usize, mut destination: &TcpStream) {
    match writeln!(destination, "{}", size) {
        Err(err) => println!("{}", err),
        Ok(()) => println!("Board size sent successfully!"),
    }
}

fn read_line(stream: &TcpStream) -> String {
    let mut input = String::new();
    BufReader::new(stream).read_line(&mut input).ok();
    return input.trim().to_string();
}
