use crate::tak_board;
use crate::types::*;

pub fn start_game() {
    let mut board = tak_board::TakBoard::new(tak_board::BoardSize::Size5x5);
    board.put_stone(Position::new(2, 3), tak_board::StoneType::CapStone);
    board.put_stone(Position::new(1, 3), tak_board::StoneType::FlatStone);
    board.put_stone(Position::new(1, 2), tak_board::StoneType::StandingStone);
    println!("{board}");
    // TODO: implement the game loop
}

/// Represents all the info that Tak move requires
enum TakInput {
    /// Put either a flat, standing or a cap stone
    PutStone {
        /// the cell to put the new stone at
        cell_position: Position,
        stone_type: i32,
    },
    /// Move a stack or a single stone toward a direction
    MoveStack {
        /// This is the initial cell of the stack
        cell_position: Position,
        /// How many stone from the top to carry
        carry_amount: i32,
        /// The direction to break apart the selected stack towards
        direction: Direction,
    },
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TakInput {
    /// Parse an input string into a new _TakInput_
    ///
    /// The format should be either:
    ///     - `<position><amount><direction>`
    ///     - `<position>[stone_type]`
    ///
    /// where:
    ///     - `position`: should contain a **row** number and a **column** letter (ex: g2, 4d)
    ///     - `amount`: is a positive number > 1
    ///     - `direction`: is one of four letters (u: Up, d: Down, l: Left, r: Right)
    ///     - `stone_type`: is a letter (f: Flat, w: Wall, c: Capstone)
    ///     defaults to flat stone if omitted
    ///
    /// > Note:
    ///     the input is case **insensitive**, and all white space is ignored
    fn from_str(s: &str) -> Result<Self, &str> {
        // First extract the position:
        const VALID_ROWS: &str = "12345678";
        const VALID_COLS: &str = "abcdefgh";
        let mut chars = s.chars();
        chars.find(|c| VALID_ROWS.contains(&c.to_string()));
        Err("hello")
    }
}
