use crate::board::Board;
use crate::position::Position;
use crate::marking::Marking;

/// An agent has a name and is able to manipulate the board.
pub trait Agent {
    /// Given a board state and marking, make a move.
    fn play(&self, board: &Board, marking: &Marking) -> Position;

    /// What the palyer is called.
    fn name(&self) -> String;
}
