use tic_tac_toe::{game::GameInstance, utils::input::Play, Board, Move};

fn main() {
    let board = Board::new();
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
