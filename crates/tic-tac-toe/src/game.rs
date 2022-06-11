use std::io::{stdin, stdout, Write};

use crate::{
    utils::input::{Coord, Play},
    Board, Move, Player,
};

pub struct GameInstance<Sender: Fn((usize, usize)), Reciever: Fn(&Board) -> Option<Play>> {
    pub board: Board,
    this_player: Player,
    send_play: Sender,
    recieve_their_play: Reciever,
}

impl<S: Fn((usize, usize)), R: Fn(&Board) -> Option<Play>> GameInstance<S, R> {
    pub fn new(board: Board, move_as: Move, s: S, r: R) -> Self {
        GameInstance {
            board,
            this_player: Player::new(move_as),
            send_play: s,
            recieve_their_play: r,
        }
    }

    pub fn run(&mut self) {
        println!("{}", self.board);

        loop {
            let mut input = String::new();
            print!("move > ");
            stdout().flush().ok();
            stdin().read_line(&mut input).ok();

            if input == "clear\n" {
                self.board = Board::new();
                println!("{}", self.board);
                continue;
            }

            match Coord::parse(&input) {
                Err(err) => println!("bad: {:?}", err),
                Ok(Coord(row, col)) => match row {
                    None => println!("bad: row and column numbers are required, (e.g 1 1)"),
                    Some(row) => match col {
                        None => println!("bad: need a column number"),
                        Some(col) => {
                            match self.this_player.play(&mut self.board, row, col) {
                                Err(err) => println!("{}", err),
                                Ok(()) => {
                                    println!("\n{}", self.board);
                                    (self.send_play)((row, col));
                                    match (self.recieve_their_play)(&self.board) {
                                        None => println!("their play was invalid..."),
                                        Some(Play(m, row, col)) => {
                                            self.board.set_at(m, (row, col));
                                            println!("\n{}", self.board);
                                        }
                                    }
                                }
                            };
                        }
                    },
                },
            }
        }
    }
}
