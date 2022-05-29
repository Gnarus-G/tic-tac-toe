use std::{io::{ stdin, stdout, Write}, num::ParseIntError};

use crate::{Board, Player, Move, utils::input::Coord};

pub fn game_loop(their_play: &dyn Fn(&mut Board)) {
    let mut board = Board::new();
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

        let coord: Result<Coord, ParseIntError> =
            input.split_whitespace().map(|s| s.parse()).collect();

        match coord {
            Err(err) => println!("bad: {:?}", err),
            Ok(Coord(row, col)) => match row {
                None => println!("bad: row and column numbers are required, (e.g 1 1)"),
                Some(row) => match col {
                    None => println!("bad: need a column number"),
                    Some(col) => {
                        match x.play(&mut board, row, col) {
                            Err(err) => println!("{}", err),
                            Ok(()) => {
                                their_play(&mut board);
                                println!("\n{}", board);
                            }
                        };
                    }
                },
            },
        }
    }
}


