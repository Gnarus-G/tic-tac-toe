use std::{
    env::args,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};
use tic_tac_toe::*;
use tic_tac_toe::{game::game_loop, utils::input::Coord};

fn main() -> std::io::Result<()> {
    if choose_be_server() {
        println!("awaiting a challenger...");
        return as_server();
    } else {
        println!("looking for someone challenge");
        return as_client();
    }
}

fn as_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("|- challenger found!\n");

    game_loop(Board::new(), &|t| send_our_play(t, &stream), &|board| {
        if let Err(err) = capture_their_play(board, &stream) {
            println!("{}", err);
        }
    });

    Ok(())
}

fn capture_their_play(board: &mut Board, stream: &TcpStream) -> std::io::Result<()> {
    println!("waiting for their play...\n");
    let mut input = String::new();
    BufReader::with_capacity(5, stream).read_line(&mut input)?;
    let coord = Coord::parse(&input);
    match coord {
        Err(err) => println!("error parsing their move: {}", err),
        Ok(Coord(row, col)) => board.set_at(Move::O, (row.unwrap(), col.unwrap())),
    }
    Ok(())
}

fn send_our_play((row, col): (usize, usize), mut destination: &TcpStream) {
    match writeln!(destination, "{} {}", row, col) {
        Err(err) => println!("{}", err),
        Ok(()) => println!("our play sent successfully!"),
    }
}

fn as_client() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:1234")?;
    let mut board = Board::new();

    println!("|- challenger found!\n");
    println!("{}", board);

    capture_their_play(&mut board, &stream)?;

    game_loop(board, &|t| send_our_play(t, &stream), &|board| {
        if let Err(err) = capture_their_play(board, &stream) {
            println!("{}", err);
        }
    });

    Ok(())
}

fn choose_be_server() -> bool {
    let args: Vec<String> = args().collect();
    let choice: bool = args
        .get(1)
        .map(|a| a.parse().unwrap_or_default())
        .unwrap_or_default();
    choice
}
