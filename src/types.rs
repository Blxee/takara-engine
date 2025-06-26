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
