use crate::tak_board;
use crate::types::*;
use Direction::*;
use StoneType::*;

pub fn start_game() {
    let mut board = tak_board::TakBoard::new(tak_board::BoardSize::Size5x5);
    board.put_stone(Position::new(2, 3), CapStone);
    board.put_stone(Position::new(1, 3), FlatStone);
    board.put_stone(Position::new(1, 2), StandingStone);
    println!("{board}");
    match TakInput::try_from("a12l1113") {
        Err(msg) => println!("{msg}"),
        Ok(_) => println!("parsing was successful"),
    }
    board.move_stack(Position::new(2, 3), 1, Right, Vec::new());
    println!("{board}");
    // TODO: implement the game loop
}
