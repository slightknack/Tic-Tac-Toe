use std::fmt;
use crate::position::Position;
use crate::marking::Marking;

/// Represents a basic 3x3 tic-tac-toe board.
pub struct Board([Marking; 9]);

impl Board {
    /// Create a new empty board.
    pub fn blank() -> Board {
        Board([Marking::Blank; 9])
    }

    /// Convert 2d incicies to a 1d index.
    pub fn to_index(position: &Position) -> usize {
        ((3 * position.x()) + position.y()) as usize
    }

    /// return the `Marking` of a board position.
    fn marking_at(&self, position: &Position) -> Marking {
        self.0[Board::to_index(position)]
    }

    /// Returns whether making a move at a specific position is valid.
    pub fn is_valid(&self, position: &Position) -> bool {
        if position.x() >= 3 || position.y() >= 3 { return false }  // bounds check

        match self.marking_at(position) {
            Marking::Blank => true,
            _              => false,
        }
    }

    /// Make a move at a position.
    /// Returns whether the move was made successfully.
    pub fn update(&mut self, position: &Position, marking: &Marking) -> bool {
        if !self.is_valid(position) { return false; }
        self.0[Board::to_index(position)] = *marking;
        return true;
    }

    /// Returns all the rows of the board in a 3x3 matrix.
    pub fn rows(&self) -> [[Marking; 3]; 3] {
        let mut arr = [[Marking::Blank; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                arr[i][j] = self.marking_at(&Position::new(i as u8, j as u8))
            }
        }

        return arr;
    }

    /// Like rows, but in column-major order.
    pub fn cols(&self) -> [[Marking; 3]; 3] {
        let mut arr = [[Marking::Blank; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                arr[i][j] = self.marking_at(&Position::new(j as u8, i as u8))
            }
        }

        return arr;
    }

    /// Return the two diagonals of the board.
    pub fn diags(&self) -> [[Marking; 3]; 2] {
        let mut arr = [[Marking::Blank; 3]; 2];

        for i in 0..3 {
            arr[0][i] = self.marking_at(&Position::new(i as u8,   i as u8));
            arr[1][i] = self.marking_at(&Position::new(2-i as u8, i as u8));
        }

        println!("{:?}", arr);

        return arr;
    }

    /// Return the current end-status of the board.
    /// None means game is still ongoing.
    /// `Marking::Blank` means draw,
    /// any other marking denotes the marking of the winner.
    pub fn status(&self) -> Option<Marking> {
        let mut all = vec![];
        all.append(&mut self.rows().to_vec());
        all.append(&mut self.cols().to_vec());
        all.append(&mut self.diags().to_vec());

        // check for 3 in a row
        for [a, b, c] in all.iter() {
            if a == b && b == c && a != &Marking::Blank {
                return Some(*a)
            }
        }

        // check for draw
        if self.0.iter()
            .filter(|b| *b == &Marking::Blank)
            .collect::<Vec<&Marking>>()
            .len() == 0
        {
            return Some(Marking::Blank)
        }

        return None;
    }
}

impl fmt::Display for Board {
    // Draw the board in a pretty manner.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marking = self.rows()
            .iter()
            .map(|row| format!("\t {} │ {} │ {} \n", row[0], row[1], row[2]))
            .collect::<Vec<String>>()
            .join("\t───┼───┼───\n");

        write!(f, "{}", marking)?;
        return Ok(());
    }
}
