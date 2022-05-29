use clap::Parser;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use tic_tac_toe::game::GameInstance;
use tic_tac_toe::{utils::input::Play, *};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Play tic-tac-toe over TCP/IP with your friend(s).
struct Args {
    /// Play as X; which makes you the server of the p2p connection. X goes first.
    #[clap(short)]
    x: bool,

    /// IP address of the X peer's connection. Required if you're not X.
    #[clap(short, long, required_unless_present = "x")]
    ip: Option<String>,

    /// Port number of the X peer's connection.
    #[clap(short = 'p', long, default_value = "1234")]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let ip = &args.ip.unwrap_or("localhost".to_string());
    let address = format!("{}:{}", ip, args.port);

    if args.x {
        println!("awaiting a challenger...");
        return as_server(&address);
    } else {
        println!("looking for someone challenge");
        return as_client(&address);
    }
}

fn as_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;

    fn handle_client(stream: TcpStream) -> std::io::Result<()> {
        println!("|- challenger found!\n");

        GameInstance::new(
            Board::new(),
            Move::X,
            |t| send_our_play(t, &stream),
            |_| recieve_their_play(Move::O, &stream),
        )
        .run();

        Ok(())
    }

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

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

fn as_client(address: &str) -> std::io::Result<()> {
    let stream = TcpStream::connect(address)?;
    let mut board = Board::new();

    println!("|- challenger found!\n");
    println!("{}", board);

    match recieve_their_play(Move::X, &stream) {
        None => println!("their play was invalid..."),
        Some(Play(m, tr, tc)) => board.set_at(m, (tr, tc)),
    };

    GameInstance::new(
        board,
        Move::O,
        |t| send_our_play(t, &stream),
        |_| recieve_their_play(Move::O, &stream),
    )
    .run();

    Ok(())
}