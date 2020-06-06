use crate::board::Board;
use crate::agent::Agent;
use crate::marking::Marking;

/// Represents a game between two players.
pub struct Game {
    board: Board,
    p1:    Box<dyn Agent>,
    p2:    Box<dyn Agent>,
}

impl Game {
    /// Generate a new game with two specified players.
    pub fn new(p1: Box<dyn Agent>, p2: Box<dyn Agent>) -> Game {
        Game { board: Board::blank(), p1, p2 }
    }

    /// Play a game through to completion.
    /// Consumes the `Game`.
    /// once a game is over, you can't run it again.
    pub fn run(mut self) {
        println!("player 1, {}, is playing as {}'s", self.p1.name(), Marking::Naught);
        println!("player 2, {}, is playing as {}'s", self.p2.name(), Marking::Cross);

        for turn in 0.. {
            // switch off turns
            let (agent, marking) = match turn % 2 == 0 {
                true  => (&self.p1, Marking::Naught),
                false => (&self.p2, Marking::Cross),
            };

            // display board before each turn
            println!("\n{}", self.board);
            println!("{} (playing as {}'s) is up!", agent.name(), marking);

            // have the player make a move
            let position = agent.play(&self.board, &marking);
            let worked = self.board.update(&position, &marking);

            // invalid moves forfiet turns
            if !worked {
                println!("{} submitted an invalid move, their turn has been forfieted", agent.name())
            }

            // check for ending
            if let Some(ending) = self.board.status() {
                println!("\n{}", self.board);
                println!(
                    "{}",
                    match ending {
                        e if e == marking => format!("{} won!", agent.name()),
                        l if l == Marking::Blank => "it's a draw...".to_string(),
                        _ => unreachable!("the other person can't win on your turn :\\"),
                    },
                );
                break;
            }
        }
    }
}
