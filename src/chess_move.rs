use crate::board::Board;
use crate::board::Color;
use crate::board::PieceType;
use crate::utils::convert_board_coordinate_to_idx;

// Uses UCI Notation
#[derive(Debug, Clone)]
pub struct Move {
    from: u8,                     // The index of the square the piece is moving from
    to: u8,                       // The index of the square the piece is moving to
    promotion: Option<PieceType>, // Use the Piece struct here
}

impl Move {
    pub fn new(uci_string: String) -> Move {
        let from = convert_board_coordinate_to_idx(uci_string[0..2].to_string());
        let to = convert_board_coordinate_to_idx(uci_string[2..4].to_string());
        let promotion = if uci_string.len() == 5 {
            Some(PieceType::from(uci_string[4..5].to_string()))
        } else {
            None
        };

        Move {
            from,
            to,
            promotion,
        }
    }
}

pub fn validate_move(board: &Board, m: &Move) -> bool {
    // First, check to see if a piece is at the 'from' location
    let color_to_move = board.active_color;

    // Obtain a slice of bitboards based on the active color
    let bitboards: &[u64] = match color_to_move {
        Color::White => &board.bitboards[0..6], // First 6 bitboards for White
        Color::Black => &board.bitboards[6..12], // Last 6 bitboards for Black
    };

    // dbg!("{}", color_to_move);

    // Create a bitmask for the 'from' square
    let from_bit = 1u64 << m.from;


    // Find the index of the bitboard that has the 'from' bit set
    let piece_at_from_idx = bitboards.iter().position(|&bb| bb & from_bit != 0);

    dbg!("{:?}", piece_at_from_idx);
    // If no piece is found at the 'from' square, the move is invalid
    let piece_type = match piece_at_from_idx {
        Some(idx) => match color_to_move {
            Color::White => match idx {
                0 => PieceType::Pawn,
                1 => PieceType::Knight,
                2 => PieceType::Bishop,
                3 => PieceType::Rook,
                4 => PieceType::Queen,
                5 => PieceType::King,
                _ => return false, // Invalid index
            },
            Color::Black => match idx {
                0 => PieceType::Pawn,
                1 => PieceType::Knight,
                2 => PieceType::Bishop,
                3 => PieceType::Rook,
                4 => PieceType::Queen,
                5 => PieceType::King,
                _ => return false, // Invalid index
            },
        },
        None => return false, // No piece found at 'from'
    };

    // At this point, you know the piece type at 'from'
    // You can now implement further validation based on the piece type
    // For example, checking if the move is legal for that piece

    println!(
        "Validating move: {:?} from index {} to index {}",
        piece_type, m.from, m.to
    );

    // TODO: Implement piece-specific move validation

    true
}
