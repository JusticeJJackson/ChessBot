use crate::board::Piece;

// Uses UCI Notation
#[derive(Debug, Clone)]
pub struct Move {
    from: String,
    to: String,
    promotion: Option<Piece>, // Use the Piece struct here
}
