use std::{error::Error, fmt::Display, iter};

#[derive(Debug, PartialEq)]
enum Move {
    X,
    O,
}

#[derive(Debug)]
struct Board {
    current: [[Option<Move>; 3]; 3],
}

#[derive(Debug)]
struct PlayOnCompleteBoardError;

impl Board {
    pub fn new() -> Board {
        Board {
            current: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn check_lines(&mut self) -> Vec<Vec<(usize, usize)>> {
        let row_indeces_iter = || 0..self.current.len();
        let colum_indeces_iter = || 0..self.current[0].len();
        let is_all_x = |line: &Vec<(usize, usize)>| {
            line.iter()
                .all(|(i, j)| self.current[*i][*j] == Some(Move::X))
        };
        let is_all_o = |line: &Vec<(usize, usize)>| {
            line.iter()
                .all(|(i, j)| self.current[*i][*j] == Some(Move::O))
        };

        let row_lines = self
            .current
            .iter()
            .zip(row_indeces_iter())
            .map(|(row, row_idx)| (0..row.len()).map(|col_idx| (row_idx, col_idx)).collect());

        let column_lines = row_indeces_iter()
            .map(|col_idx| {
                ((colum_indeces_iter()).map(|i| &self.current[i][col_idx]))
                    .collect::<Vec<&Option<Move>>>()
            })
            .zip(row_indeces_iter())
            .map(|(col, col_idx)| (0..col.len()).map(|row_idx| (row_idx, col_idx)).collect());

        let left_diagonal_line = row_indeces_iter().map(|n| (n, n)).collect();

        let right_diagonal_line = row_indeces_iter().rev().zip(colum_indeces_iter()).collect();

        row_lines
            .chain(column_lines)
            .chain(iter::once(left_diagonal_line))
            .chain(iter::once(right_diagonal_line))
            .filter(|l| is_all_x(l) || is_all_o(l))
            .collect()
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
    pub fn play(
        mut self,
        xo: Move,
        row: usize,
        col: usize,
    ) -> Result<(), PlayOnCompleteBoardError> {
        if self.board.is_complete() {
            return Err(PlayOnCompleteBoardError {});
        }
        self.board.current[row][col] = Some(xo);
        Ok(())
    }
}

impl Error for PlayOnCompleteBoardError {}

impl Display for PlayOnCompleteBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't play on an already complete board!")
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Board, Move, Player};

    fn x() -> Option<Move> {
        Some(Move::X)
    }
    fn o() -> Option<Move> {
        Some(Move::O)
    }

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
    fn test_play_on_on_complete_board() {
        let mut board = Board::new();
        board.current = [[x(), x(), x()], [x(), x(), x()], [x(), x(), x()]];

        let result = Player::new(&mut board).play(Move::X, 0, 0);

        assert!(result.is_err());
    }

    #[test]
    fn test_board_is_complete() {
        let mut board = Board::new();
        assert!(!board.is_complete());

        board.current = [[x(), x(), x()], [x(), x(), x()], [x(), x(), x()]];

        assert!(board.is_complete());

        board.current = [[x(), None, x()], [x(), x(), x()], [x(), x(), x()]];

        assert!(!board.is_complete());

        board.current = [[x(), x(), x()], [x(), None, x()], [x(), x(), x()]];

        assert!(!board.is_complete());
    }

    #[test]
    fn test_no_winning_lines_on_blank_board() {
        let mut board = Board::new();
        let no_lines: Vec<Vec<(usize, usize)>> = vec![];
        assert_eq!(board.check_lines(), no_lines);
    }

    #[test]
    fn test_winning_rows() {
        let mut board = Board::new();

        board.current = [[x(), x(), x()], [o(), o(), x()], [o(), x(), o()]];
        assert_eq!(board.check_lines(), vec![vec![(0, 0), (0, 1), (0, 2)]]);

        board.current = [[o(), x(), o()], [o(), o(), x()], [x(), x(), x()]];
        assert_eq!(board.check_lines(), vec![vec![(2, 0), (2, 1), (2, 2)]]);
    }

    #[test]
    fn test_winning_lines_columns() {
        let mut board = Board::new();

        board.current = [[o(), x(), x()], [x(), x(), o()], [o(), x(), x()]];
        assert_eq!(board.check_lines(), vec![vec![(0, 1), (1, 1), (2, 1)]]);

        board.current = [[o(), x(), x()], [x(), x(), o()], [o(), x(), x()]];
        assert_eq!(board.check_lines(), vec![vec![(0, 1), (1, 1), (2, 1)]]);
    }
    #[test]
    fn test_winning_column_and_row() {
        let mut board = Board::new();

        board.current = [[o(), x(), x()], [o(), o(), x()], [x(), x(), x()]];
        assert_eq!(
            board.check_lines(),
            vec![vec![(2, 0), (2, 1), (2, 2)], vec![(0, 2), (1, 2), (2, 2)]]
        );
    }
    #[test]
    fn test_winning_left_diagonal() {
        let mut board = Board::new();

        board.current = [[o(), x(), x()], [o(), o(), x()], [x(), o(), o()]];
        assert_eq!(board.check_lines(), vec![vec![(0, 0), (1, 1), (2, 2)]]);
    }

    #[test]
    fn test_winning_right_diagonal() {
        let mut board = Board::new();
        board.current = [[o(), x(), x()], [o(), x(), x()], [x(), o(), o()]];
        assert_eq!(board.check_lines(), vec![vec![(2, 0), (1, 1), (0, 2)]]);
    }
}
