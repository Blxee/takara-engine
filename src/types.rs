use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    string,
};

/// Position of a cell in the tak board
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

impl TryFrom<&str> for Position {
    type Error = &'static str;

    /// Convert a position string to a Position struct
    ///
    /// # Arguments:
    ///
    /// * `value` - a string which contains a digit for the row
    /// and a letter for the column
    /// ---
    /// > Note:
    /// any characters which do not represent either row or col will be ignored.
    /// ony the first row and first col matched will be used.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        const VALID_ROWS: &str = "12345678";
        const VALID_COLS: &str = "abcdefgh";
        let value = value.to_lowercase();
        let Some(row) = value.chars().find_map(|c| VALID_ROWS.find(c)) else {
            return Err("no valid row was found");
        };
        let Some(col) = value.chars().find_map(|c| VALID_COLS.find(c)) else {
            return Err("no valid column was found");
        };
        Ok(Position::new(row as i32, col as i32))
    }
}

#[derive(Clone, Copy)]
pub enum StoneType {
    FlatStone,
    StandingStone,
    CapStone,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// Represents all the info that Tak move requires
pub enum TakInput {
    /// Put either a flat, standing or a cap stone
    PutStone {
        /// the cell to put the new stone at
        position: Position,
        stone_type: StoneType,
    },
    /// Move a stack or a single stone toward a direction
    MoveStack {
        /// This is the initial cell of the stack
        position: Position,
        /// How many stone from the top to carry
        count: u32,
        /// The direction to break apart the selected stack towards
        direction: Direction,
        /// How many stones to drop at each upcoming cell
        drops: Vec<u32>,
    },
}

impl TryFrom<&str> for TakInput {
    type Error = &'static str;

    /// Parse an input string into a new _TakInput_
    ///
    /// The format should be either:
    ///     * `<position><count><direction>[drops]..`
    ///     * `<position>[stone_type]`
    ///
    /// where:
    ///     * `position` -  should contain a **row** number and a **column** letter (ex: g2, 4d)
    ///     * `count` - is a positive number > 1
    ///     * `direction` - is one of four letters (u: Up, d: Down, l: Left, r: Right)
    ///     * `drops` - are digits represeting how many stones to drop at each cell while moving
    ///     * `stone_type` - is a letter (f: Flat, w: Wall, c: Capstone)
    ///     defaults to flat stone if omitted
    ///
    /// > Note:
    ///     the input is case **insensitive**, and all white space is ignored
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.to_lowercase();
        let position = Self::parse_position(&mut value)?;
        if let Ok(stone_type) = Self::parse_stone_type(&mut value) {
            return Ok(Self::PutStone {
                position,
                stone_type,
            });
        }
        let count = Self::parse_count(&mut value).unwrap_or(1);
        let direction = Self::parse_direction(&mut value)?;
        let drops = Self::parse_drops(&mut value);
        Ok(Self::MoveStack {
            position,
            count,
            direction,
            drops,
        })
    }
}

impl TakInput {
    /// Extract and remove the row number and column letter from the string
    fn parse_position(value: &mut String) -> Result<Position, &'static str> {
        const VALID_ROWS: &str = "12345678";
        const VALID_COLS: &str = "abcdefgh";

        let mut row = None;
        for (i, chr) in value.char_indices() {
            if let Some(idx) = VALID_ROWS.find(chr) {
                row = Some(idx as i32);
                value.remove(i);
                break;
            }
        }

        let mut col = None;
        for (i, chr) in value.char_indices() {
            if let Some(idx) = VALID_COLS.find(chr) {
                col = Some(idx as i32);
                value.remove(i);
                break;
            }
        }

        Ok(Position::new(
            row.ok_or("no valid row was found")?,
            col.ok_or("no valid column was found")?,
        ))
    }

    /// Extract and remove the stone type letter
    /// (f: for flatstone, w: for wall aka standing stone, c: for capstone)
    fn parse_stone_type(value: &mut String) -> Result<StoneType, &'static str> {
        for (i, chr) in value.char_indices() {
            let stone_type = match chr {
                'f' => StoneType::FlatStone,
                'w' => StoneType::StandingStone,
                'c' => StoneType::CapStone,
                _ => continue,
            };
            value.remove(i);
            return Ok(stone_type);
        }
        Err("no stone letter was found")
    }

    /// Extract and remove the count digit from the string
    fn parse_count(value: &mut String) -> Option<u32> {
        for (i, chr) in value.char_indices() {
            if let Some(count) = chr.to_digit(10) {
                value.remove(i);
                return Some(count as u32);
            };
        }
        None
    }

    /// Extract and remove the direction letter from the string
    /// (l: left, r: right, u: up, d: down)
    fn parse_direction(value: &mut String) -> Result<Direction, &'static str> {
        for (i, chr) in value.char_indices() {
            let direction = match chr {
                'u' => Direction::Up,
                'd' => Direction::Down,
                'l' => Direction::Left,
                'r' => Direction::Right,
                _ => continue,
            };
            value.remove(i);
            return Ok(direction);
        }
        Err("no direction letter was found")
    }

    /// Extract digits represeting amount to drop at each cell while moving
    fn parse_drops(value: &mut String) -> Vec<u32> {
        // BUG: this removes all the matching digits
        // without accounting to the offset after each remove
        let mut drops = Vec::new();
        for (i, chr) in value.clone().char_indices() {
            if let Some(count) = chr.to_digit(10) {
                // INFO: since this is the last part of tak input
                // this remove is not necessary
                value.remove(i);
                drops.push(count);
            };
        }
        drops
    }
}
