use std::io::{stdin, stdout, Write};

use crate::{utils::input::Coord, Board, Move, Player};

pub fn game_loop(
    mut board: Board,
    send_our_play: &dyn Fn((usize, usize)),
    recieve_their_play: &dyn Fn(&mut Board),
) {
    let mut x = Player::new(Move::X);

    println!("{}", board);

    loop {
        let mut input = String::new();
        print!("move > ");
        stdout().flush().ok();
        stdin().read_line(&mut input).ok();

        if input == "clear\n" {
            board = Board::new();
            println!("{}", board);
            continue;
        }

        match Coord::parse(&input) {
            Err(err) => println!("bad: {:?}", err),
            Ok(Coord(row, col)) => match row {
                None => println!("bad: row and column numbers are required, (e.g 1 1)"),
                Some(row) => match col {
                    None => println!("bad: need a column number"),
                    Some(col) => {
                        match x.play(&mut board, row, col) {
                            Err(err) => println!("{}", err),
                            Ok(()) => {
                                println!("\n{}", board);
                                send_our_play((row, col));
                                recieve_their_play(&mut board);
                                println!("\n{}", board);
                            }
                        };
                    }
                },
            },
        }
    }
}
