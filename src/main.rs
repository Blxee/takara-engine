fn main() {
    println!("Hello, world!");
}

mod tak_board {
    use std::fmt::{self};

    struct TakBoard<const N: usize = 5> {
        grid: [[Cell<N>; N]; N],
    }

    struct Cell<const N: usize = 5> {
        stack: [Option<Piece>; N],
    }

    #[derive(Clone, Copy)]
    struct Piece {
        color: PieceColor,
        typ: PieceType,
    }

    #[derive(Clone, Copy)]
    enum PieceColor {
        White,
        Black,
    }

    #[derive(Clone, Copy)]
    enum PieceType {
        FlatStone,
        StandingStone,
        CapStone,
    }

    impl TakBoard {
        const fn new() -> Self {
            let mut grid = [const {
                [const { Cell::new() }; 5];
                5
            }];
            Self { grid }
        }
    }

    impl fmt::Display for TakBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //      a     b     c     d     e
            //   +-----+-----+-----+-----+-----+
            // 1 | F   |FFF  |     |     |     | 1
            //   +-----+-----+-----+-----+-----+
            // 2 | W   |FFFW |     |     |     | 2
            //   +-----+-----+-----+-----+-----+
            // 3 |     |FFC  |     |     |     | 3
            //   +-----+-----+-----+-----+-----+
            // 4 |     |     |     |     |     | 4
            //   +-----+-----+-----+-----+-----+
            // 5 |     |     |     |     |     | 5
            //   +-----+-----+-----+-----+-----+
            //      a     b     c     d     e

            write!(f, "")
        }
    }

    impl Cell {
        const fn new() -> Self {
            Self { stack: [None; 5] }
        }
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut repr = String::with_capacity(self.stack.len());
            // for each layer in the cell stack
            for layer in self.stack.iter() {
                // append the repr of each piece and fill the rest with space
                repr.push_str(
                    layer
                        .as_ref()
                        .map_or(" ".to_string(), |piece| piece.to_string())
                        .as_str(),
                );
            }

            write!(f, "{repr}")
        }
    }

    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.typ {
                PieceType::FlatStone => write!(f, "F"),
                PieceType::StandingStone => write!(f, "W"),
                PieceType::CapStone => write!(f, "C"),
            }
        }
    }
}
