use std::fmt::{self};

// enum TakBoardConfig {
//     Two = 2,
//     Three = 3,
//     Four = 4,
//     Five = 5,
//     Six = 6,
//     Seven = 7,
//     Eight = 8,
// }

pub struct TakBoard<const N: usize = 5> {
    grid: [[Cell<N>; N]; N],
    players: [TakPlayer; 2],
    turn: StoneColor,
}

#[derive(Clone, Copy)]
struct Cell<const N: usize = 5> {
    stack: [Option<Stone>; N],
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

impl<const N: usize> TakBoard<N> {
    pub const fn new() -> Self {
        Self {
            grid: [[Cell::new(); N]; N],
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
        for (i, row) in self.grid.iter().enumerate() {
            write!(f, " {} |", i + 1)?;
            for cell in row {
                write!(f, "{cell}|")?;
            }
            writeln!(f, " {}\n   +-----+-----+-----+-----+-----+", i + 1)?;
        }
        write!(f, "      a     b     c     d     e")
    }
}

impl<const N: usize> Cell<N> {
    const fn new() -> Self {
        Self { stack: [None; N] }
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
