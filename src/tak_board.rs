use std::{
    collections::HashMap,
    fmt::{self},
};

enum TakBoardSize {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

struct TakBoardConfig {
    board_size: u32,
    normal_stones: u32,
    capstones: u32,
}

pub struct TakBoard {
    grid: HashMap<(i32, i32), Cell>,
    players: [TakPlayer; 2],
    turn: StoneColor,
}

struct Cell {
    stack: Vec<Stone>,
}

#[derive(Clone, Copy)]
struct Stone {
    color: StoneColor,
    typ: StoneType,
}

#[derive(Clone, Copy)]
enum StoneColor {
    White = 0,
    Black = 1,
}

#[derive(Clone, Copy)]
enum StoneType {
    FlatStone,
    StandingStone,
    CapStone,
}

struct TakPlayer {
    color: StoneColor,
    stones_left: u32,
    capstones_available: u32,
}

impl TakBoard {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            players: [TakPlayer::new(), TakPlayer::new()],
            turn: StoneColor::White,
        }
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

        writeln!(f, "      a     b     c     d     e")?;
        writeln!(f, "   +-----+-----+-----+-----+-----+")?;
        for row in 0..5 {
            write!(f, " {} |", row + 1)?;
            for col in 0..5 {
                let cell = self.grid.get(&(row, col)).unwrap();
                write!(f, "{cell}|")?;
            }
            writeln!(f, " {}\n   +-----+-----+-----+-----+-----+", row + 1)?;
        }
        write!(f, "      a     b     c     d     e")
    }
}

impl Cell {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::with_capacity(self.stack.len());
        // for each stone in the cell stack
        for stone in self.stack.iter() {
            // append the repr of each piece
            repr.push_str(&stone.to_string());
        }

        // TODO: the stack will distort the shape of the board!!

        write!(f, "{repr}")
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.typ {
            StoneType::FlatStone => write!(f, "F"),
            StoneType::StandingStone => write!(f, "W"),
            StoneType::CapStone => write!(f, "C"),
        }
    }
}

impl TakPlayer {
    const fn new() -> Self {
        Self {
            color: StoneColor::White,
            stones_left: 22,
            capstones_available: 1,
        }
    }
}
