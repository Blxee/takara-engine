fn main() {
    println!("Hello, world!");
}

mod tak_board {
    struct TakBoard<const N: usize = 5> {
        size: i32,
        grid: [[Cell<N>; N]; N],
    }

    struct Cell<const N: usize = 5> {
        stack: [Piece; N],
    }

    struct Piece {
        color: PieceColor,
        typ: PieceType,
    }

    enum PieceColor {
        White,
        Black,
    }

    enum PieceType {
        FlatStone,
        StandingStone,
        CapStone,
    }

    impl TakBoard {
        const fn new() -> Self {
            Self {
                size: todo!(),
                grid: todo!(),
            }
        }
    }
}
