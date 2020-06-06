use std::fmt;

/// Represents a marking on a board,
/// whether a naught, a cross, or a blank tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marking {
    Blank,
    Naught,
    Cross,
}

/// Draws the symbolic representation for a given marking.
impl fmt::Display for Marking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Marking::Blank  => " ",
            Marking::Naught => "○",
            Marking::Cross  => "⨯",
        })
    }
}
