use std::{
    env::args,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use tic_tac_toe::game::GameInstance;
use tic_tac_toe::{utils::input::Play, *};

const PORT: u16 = 1234;

fn main() -> std::io::Result<()> {
    if choose_be_server() {
        println!("awaiting a challenger...");
        return as_server();
    } else {
        println!("looking for someone challenge");
        return as_client("localhost");
    }
}

fn as_server() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("localhost:{}", PORT))?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("|- challenger found!\n");

    GameInstance::new(
        Board::new(),
        Move::X,
        |t| send_our_play(t, &stream),
        |_| recieve_their_play(Move::O, &stream),
    ).run();

    Ok(())
}

fn recieve_their_play(m: Move, stream: &TcpStream) -> Option<Play> {
    println!("waiting for their play...\n");
    let mut input = String::new();
    BufReader::with_capacity(5, stream)
        .read_line(&mut input)
        .ok();
    Play::from(&input, m)
}

fn send_our_play((row, col): (usize, usize), mut destination: &TcpStream) {
    match writeln!(destination, "{} {}", row, col) {
        Err(err) => println!("{}", err),
        Ok(()) => println!("our play sent successfully!"),
    }
}

fn as_client(server_ip: &str) -> std::io::Result<()> {
    let stream = TcpStream::connect(format!("{}:{}", server_ip, PORT))?;
    let mut board = Board::new();

    println!("|- challenger found!\n");
    println!("{}", board);

    match recieve_their_play(Move::X, &stream) {
        None => println!("their play was invalid..."),
        Some(Play(m, tr, tc)) => board.set_at(m, (tr, tc))
    };

    GameInstance::new(
        board,
        Move::O,
        |t| send_our_play(t, &stream),
        |_| recieve_their_play(Move::O, &stream),
    ).run();

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
