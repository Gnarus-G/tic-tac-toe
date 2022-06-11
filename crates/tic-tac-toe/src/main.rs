use clap::Parser;
use tic_tac_toe::{game::GameInstance, utils::input::Play, Board, Move};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Play tic-tac-toe against a very stupid bot.
struct Args {
    /// Size of the board.
    #[clap(short, long, default_value = "3")]
    size: usize,
}

fn main() {
    let args = Args::parse();
    let board = Board::with_size(args.size);
    GameInstance::new(board, Move::X, |_| {}, bot_play).run();
}

fn bot_play(board: &Board) -> Option<Play> {
    let mut move_at = (0, 0);
    'outer: for (i, row) in board.rows().enumerate() {
        for (j, m) in row.iter().enumerate() {
            if let None = m {
                move_at = (i, j);
                break 'outer;
            }
        }
    }
    let (row, col) = move_at;
    Some(Play(Move::O, row, col))
}
