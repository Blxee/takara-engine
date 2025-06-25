use std::{
    collections::HashMap,
    fmt::{self},
};
use rand::{rng, seq::IndexedRandom};

enum TakBoardSize {
    Size3x3,
    Size4x4,
    Size5x5,
    Size6x6,
    Size7x7,
    Size8x8,
}
use TakBoardSize::*;

pub struct TakBoard {
    size: usize,
    grid: HashMap<(usize, usize), Cell>,
    players: [TakPlayer; 2],
    turn: StoneColor,
}

struct Cell {
    stack: Vec<Stone>,
}

#[derive(Clone, Copy)]
struct Stone {
    color: StoneColor,
    stone_type: StoneType,
}

#[derive(Clone, Copy)]
enum StoneColor {
    White = 0,
    Black = 1,
}
use StoneColor::*;

#[derive(Clone, Copy)]
enum StoneType {
    FlatStone,
    StandingStone,
    CapStone,
}
use StoneType::*;

struct TakPlayer {
    color: StoneColor,
    stones_available: u32,
    capstones_available: u32,
}

impl TakBoard {
    pub fn new(size: TakBoardSize) -> Self {
        let (size, stones_available, capstones_available) = match size {
            Size3x3 => (3, 10, 0),
            Size4x4 => (4, 15, 0),
            Size5x5 => (5, 20, 1),
            Size6x6 => (6, 30, 1),
            Size7x7 => (7, 40, 2),
            Size8x8 => (8, 50, 2),
        };
        // Fill the cells of the grid
        let mut grid = HashMap::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                grid.insert((row, col), Cell::new());
            }
        }

        Self {
            size,
            grid,
            players: [
                TakPlayer {
                    color: White,
                    stones_available,
                    capstones_available,
                },
                TakPlayer {
                    color: Black,
                    stones_available,
                    capstones_available,
                }
            ],
            turn: *[White, Black].choose(&mut rng()).unwrap(),
        }
    }

    fn swap_turns(&mut self) {
        self.turn = match self.turn {
            White => Black,
            Black => White,
        };
    }

    /// Put a new stone on the board
    fn put_stone(
        &mut self,
        position: (usize, usize),
        stone_type: StoneType,
    ) -> Result<(), &'static str> {
        let current_player = &mut self.players[self.turn as usize];
        // Make sure player has enough of this stone type
        match stone_type {
            CapStone => {
                if current_player.capstones_available <= 0 {
                    return Err("");
                }
            }
            FlatStone | StandingStone => {
                if current_player.stones_available <= 0 {
                    return Err("");
                }
            }
        }
        // Only put stone if cell is empty
        let cell = self
            .grid
            .get_mut(&position)
            .expect("Board was not initialized currectly");
        if !cell.stack.is_empty() {
            return Err("");
        }
        // Put stone in the cell
        cell.stack.push(Stone {
            color: self.turn,
            stone_type,
        });
        // Subtract the stone we just put on board
        match stone_type {
            CapStone => current_player.capstones_available -= 1,
            FlatStone | StandingStone => current_player.stones_available -= 1,
        }
        self.swap_turns();
        Ok(())
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
                let cell_repr = self
                    .grid
                    .get(&(row, col))
                    .map_or(" ".repeat(5), |cell| cell.to_string());
                write!(f, "{cell_repr}|")?;
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
        match self.stone_type {
            StoneType::FlatStone => write!(f, "F"),
            StoneType::StandingStone => write!(f, "W"),
            StoneType::CapStone => write!(f, "C"),
        }
    }
}
