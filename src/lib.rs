use std::{fmt::Display, iter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Move {
    X,
    O,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::X => write!(f, "X"),
            Move::O => write!(f, "O"),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    current: [[Option<Move>; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            current: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn set_at(&mut self, m: Move, (row, col): (usize, usize)) {
        self.current[row][col] = Some(m);
    }

    pub fn rows(&self) -> std::slice::Iter<[Option<Move>; 3]> {
        self.current.iter()
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
            .enumerate()
            .map(|(row_idx, row)| (0..row.len()).map(|col_idx| (row_idx, col_idx)).collect());

        let column_lines = row_indeces_iter()
            .map(|col_idx| {
                ((colum_indeces_iter()).map(|i| &self.current[i][col_idx]))
                    .collect::<Vec<&Option<Move>>>()
            })
            .enumerate()
            .map(|(col_idx, col)| (0..col.len()).map(|row_idx| (row_idx, col_idx)).collect());

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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut disp = String::new();

        //write column number labels
        disp.push_str("  |");
        for i in 0..self.current.len() {
            disp.push_str(&format!(" {} ", i));
        }
        disp.push('\n');
        
        //adding an extra line of separators
        disp.push_str(" --");
        for _ in 0..self.current.len() {
            disp.push_str(&format!("---"));
        }
        disp.push('\n');
 
        //write each row starting with row index label
        for (i, row) in self.current.iter().enumerate() {
            disp.push_str(&format!(" {}|", i));
            for m in row {
                match m {
                    None => disp.push_str(" _ "),
                    Some(m) => disp.push_str(&format!(" {} ", m)),
                }
            }
            disp.push('\n');
        }

        write!(f, "{}", disp)
    }
}

pub struct Player {
    move_as: Move,
}

impl Player {
    pub fn new(move_as: Move) -> Player {
        Player { move_as }
    }
    pub fn play(&mut self, board: &mut Board, row: usize, col: usize) {
        board.set_at(self.move_as, (row, col));
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
        let mut x = Player::new(Move::X);
        x.play(&mut board, 0, 0);
        x.play(&mut board, 2, 1);

        let mut o = Player::new(Move::O);
        o.play(&mut board, 1, 2);

        assert_eq!(board.current[0][0], Some(Move::X));
        assert_eq!(board.current[2][1], Some(Move::X));
        assert_eq!(board.current[1][2], Some(Move::O));
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
