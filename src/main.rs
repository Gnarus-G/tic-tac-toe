use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

use tic_tac_toe::{Board, Move, Player};

struct Coord(Option<usize>, Option<usize>);

impl FromIterator<usize> for Coord {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        Coord(i.next(), i.next())
    }
}

fn main() {
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
                                bot_play(&mut board);
                                println!("\n{}", board);
                            }
                        };
                    }
                },
            },
        }
    }
}

fn bot_play(board: &mut Board) {
    let mut move_at = (0, 0);
    'outer: for (i, row) in board.rows().enumerate() {
        for (j, m) in row.iter().enumerate() {
            if let None = m {
                move_at = (i, j);
                break 'outer;
            }
        }
    }
    return board.set_at(Move::O, move_at);
}
