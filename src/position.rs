/// Represents a position (row, column), on the board.
pub struct Position(u8, u8);

impl Position {
    /// Make a new position.
    pub fn new(x: u8, y: u8) -> Position {
        Position(x, y)
    }

    /// Get the row
    pub fn x(&self) -> u8 {
        return self.0
    }

    /// Get the column.
    pub fn y(&self) -> u8 {
        return self.1
    }
}
