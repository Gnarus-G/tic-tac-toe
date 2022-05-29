use tic_tac_toe::{game::game_loop, Board, Move};

fn main() {
    game_loop(Board::new(), &|_| {}, &bot_play)
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
