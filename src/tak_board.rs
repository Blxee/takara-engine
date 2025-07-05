use crate::types::*;
use rand::{rng, seq::IndexedRandom};
use std::{
    collections::HashMap,
    fmt::{self},
};
use Direction::*;
use StoneType::*;

pub enum BoardSize {
    Size3x3,
    Size4x4,
    Size5x5,
    Size6x6,
    Size7x7,
    Size8x8,
}
use BoardSize::*;

pub struct TakBoard {
    size: u32,
    grid: HashMap<Position, Cell>,
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

struct TakPlayer {
    color: StoneColor,
    stones_available: u32,
    capstones_available: u32,
}

impl TakBoard {
    pub fn new(size: BoardSize) -> Self {
        let (size, stones_available, capstones_available): (u32, u32, u32) = match size {
            Size3x3 => (3, 10, 0),
            Size4x4 => (4, 15, 0),
            Size5x5 => (5, 20, 1),
            Size6x6 => (6, 30, 1),
            Size7x7 => (7, 40, 2),
            Size8x8 => (8, 50, 2),
        };
        // Fill the cells of the grid
        let mut grid = HashMap::with_capacity((size * size) as usize);
        for row in 0..size {
            for col in 0..size {
                grid.insert(Position::new(row as i32, col as i32), Cell::new());
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
                },
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
    pub fn put_stone(
        &mut self,
        position: Position,
        stone_type: StoneType,
    ) -> Result<(), &'static str> {
        let current_player = &mut self.players[self.turn as usize];
        // Make sure player has enough of this stone type
        match stone_type {
            CapStone => {
                if current_player.capstones_available <= 0 {
                    return Err("There are no capstones left");
                }
            }
            FlatStone | StandingStone => {
                if current_player.stones_available <= 0 {
                    return Err("There are no normal stones left");
                }
            }
        }
        // Only put stone if cell is empty
        let cell = self
            .grid
            .get_mut(&position)
            .expect("Board was not initialized currectly");
        if !cell.stack.is_empty() {
            return Err("This cell is not empty");
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

    // TODO: split apart and refactor this amalgamation of a function

    /// Moves an amount of stones from the top of the stack towards a direction
    ///
    /// # Arguments:
    ///
    /// * `position` - the cell position of the original stack
    /// * `carry_amount` - the amount of stones to carry from the top
    /// (up to max limit of board size) defaults to `1` if ommited
    /// * `direction` - the direction to break apart the carried towards
    /// * `stacks` - how many stones to leave at each step, defaults to 1 at each cell then leave
    /// the rest at the last cell possible
    pub fn move_stack(
        &mut self,
        position: Position,
        count: usize,
        direction: Direction,
        stacks: Vec<usize>,
    ) -> Result<(), &'static str> {
        // WARN: standing stones should not be moved
        // set default stacks to 1 stone to be put for each cell passed
        let stacks = vec![1].repeat(self.size as usize);
        // get the stack at cell position
        let original_stack = &mut self
            .grid
            .get_mut(&position)
            .ok_or("position is out of board bounds")?
            .stack;
        // if user is trying to carry more than available, return err
        if count > original_stack.len() {
            return Err("cannot carry more than the original stack length");
        }
        // take the carry amount of stones from the cell
        let mut stack_to_move: Vec<_> = original_stack
            .splice((original_stack.len() - count)..original_stack.len(), [])
            .collect();
        // convert the direction to vector format
        let step: Position = direction.into();
        // move the stack towards direction while puting stones at each cell passed
        // according to stacks argument
        let mut current_position = position + step;
        for i in stacks {
            // if this cell is the furthest we can reach
            // (if we are at the border or the next head of stack is not passable)
            if !self.is_cell_passable(current_position, step, &stack_to_move) {
                // empty the whole stack here
                let cell = self.grid.get_mut(&current_position).unwrap();
                cell.stack.append(&mut stack_to_move);
                break;
            }
            // empty part of the stack here then continue
            let cell = self.grid.get_mut(&current_position).unwrap();
            cell.stack
                .append(&mut stack_to_move.splice(0..i, []).collect());
            if stack_to_move.is_empty() {
                break;
            }
            current_position += step;
        }
        Ok(())
    }

    fn top_stone_at(&self, pos: Position) -> Option<&Stone> {
        self.grid.get(&pos)?.stack.last()
    }

    /// Determines whether the next cell
    /// could be passed by the moving stack
    ///
    /// # Arguments:
    ///
    /// * `pos` - the current moving stack position
    /// * `step` - the direction of the stack movement
    /// * `stack` - the current moving stack itself
    fn is_cell_passable(&self, pos: Position, step: Position, stack: &Vec<Stone>) -> bool {
        let is_cell_at_border = self.grid.contains_key(&(pos + step));

        if is_cell_at_border {
            return true;
        }

        let Some(current_stack_head) = stack.last() else {
            return false;
        };
        let Some(next_stack_head) = self.top_stone_at(pos + step) else {
            return true;
        };

        match (current_stack_head.stone_type, next_stack_head.stone_type) {
            (_, FlatStone) => true,
            (CapStone, StandingStone) if stack.len() == 1 => true,
            _ => false,
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
                let cell_repr = self
                    .grid
                    .get(&Position::new(row, col))
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
        repr.push_str(&" ".repeat(5 - self.stack.len()));

        // WARN: the stack will distort the shape of the board!!

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
