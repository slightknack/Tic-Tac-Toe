use crate::agent::Agent;
use crate::position::Position;
use crate::board::Board;
use crate::marking::Marking;
use crate::input;

/// A `Human` is an agent controlled by a human.
/// Agents can also be AI.
pub struct Human(String);

impl Human {
    /// Create a `Human` agent, proxying for a real human.
    pub fn proxy(name: String) -> Human {
        Human(name)
    }

    /// Prompt player to make a move until a valid one is chosen.
    fn get_move(_board: &Board) -> Option<Position> {
        let r = input::read("where would you like to move? (enter '<row> <col>', zero indexed): ");
        let split = r.split(" ")
            .map(|n| n.parse::<u8>().ok())
            .collect::<Vec<Option<u8>>>();

        if split.len() != 2 { return None; }
        let (x, y) = (split[0]?, split[1]?);

        return Some(Position::new(x, y));
    }
}

impl Agent for Human {
    fn play(&self, board: &Board, _marking: &Marking) -> Position {
        // force humans to play valid moves
        loop {
            match Human::get_move(board) {
                Some(p) if board.is_valid(&p) => return p,
                _ => println!("invalid move, try again!"),
            }
        }
    }

    fn name(&self) -> String {
        return self.0.clone();
    }
}
