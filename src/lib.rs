#[derive(Debug, PartialEq)]
enum Move {
    X,
    O,
}

#[derive(Debug)]
struct Board {
    current: [[Option<Move>; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            current: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    fn is_complete(&self) -> bool {
        self.current.iter().flatten().all(|m| m.is_some())
    }
}

struct Player<'a> {
    board: &'a mut Board,
}

impl<'a> Player<'a> {
    pub fn new(board: &'a mut Board) -> Player<'a> {
        Player { board }
    }
    pub fn play(mut self, xo: Move, row: usize, col: usize) {
        self.board.current[row][col] = Some(xo);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Move, Player};

    #[test]
    fn test_play_at_position() {
        let mut board = Board::new();

        Player::new(&mut board).play(Move::X, 0, 0);
        assert_eq!(board.current[0][0], Some(Move::X));

        Player::new(&mut board).play(Move::X, 2, 1);
        assert_eq!(board.current[2][1], Some(Move::X));

        Player::new(&mut board).play(Move::O, 1, 2);
        assert_eq!(board.current[1][2], Some(Move::O));
    }

    #[test]
    fn test_board_is_complete() {
        let mut board = Board::new();
        assert!(!board.is_complete());

        let x = || Some(Move::X);

        board.current = [[x(), x(), x()], [x(), x(), x()], [x(), x(), x()]];

        assert!(board.is_complete());

        board.current = [[x(), None, x()], [x(), x(), x()], [x(), x(), x()]];

        assert!(!board.is_complete());

        board.current = [[x(), x(), x()], [x(), None, x()], [x(), x(), x()]];

        assert!(!board.is_complete());
    }
}
