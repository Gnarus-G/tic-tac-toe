pub mod error {
    #[derive(Debug)]
    pub struct InvalidPlayError;

    impl std::error::Error for InvalidPlayError {}

    impl std::fmt::Display for InvalidPlayError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Can't play there!")
        }
    }
}

pub mod input {
    use std::num::ParseIntError;

    use crate::Move;

    #[derive(Debug)]
    pub struct Coord(pub Option<usize>, pub Option<usize>);

    impl FromIterator<usize> for Coord {
        fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
            let mut i = iter.into_iter();
            Coord(i.next(), i.next())
        }
    }

    impl Coord {
        pub fn parse(input: &str) -> Result<Coord, ParseIntError> {
            input.split_whitespace().map(|s| s.parse()).collect()
        }
    }

    pub struct Play(pub Move, pub usize, pub usize);

    impl Play {
        pub fn from(input: &str, move_as: Move) -> Option<Play> {
            let result = input.split_whitespace().map(|s| s.parse()).collect();
            match result {
                Err(_) => None,
                Ok(Coord(row, col)) => match row {
                    None => None,
                    Some(row) => match col {
                        None => None,
                        Some(col) => Some(Play(move_as, row, col))
                    },
                },
            }
        }
    }
}
