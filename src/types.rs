use std::ops::{Add, AddAssign, Sub, SubAssign};

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
