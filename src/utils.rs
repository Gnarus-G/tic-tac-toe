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
    pub struct Coord(pub Option<usize>, pub Option<usize>);

    impl FromIterator<usize> for Coord {
        fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
            let mut i = iter.into_iter();
            Coord(i.next(), i.next())
        }
    }
}
