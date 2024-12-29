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
    let piece_type = match find_peice_at_from_location(board, m) {
        Some(pt) => pt,
        None => {
            println!("No piece found at '{}'", m.from);
            return false;
        } // No piece found at 'from'
    };

    // Validate the move based on the piece type
    // Note: We check if the 'to' location is valid inside the validate functions to handle edge cases (e.g. castling)
    let valid_move = match piece_type {
        PieceType::Pawn => validate_pawn_move(board, m),
        PieceType::Knight => validate_knight_move(board, m),
        PieceType::Bishop => validate_bishop_move(board, m),
        PieceType::Rook => validate_rook_move(board, m),
        PieceType::Queen => validate_queen_move(board, m),
        PieceType::King => validate_king_move(board, m),
    };

    if valid_move {
        println!(
            "Valid move: {:?} from index {} to index {}",
            piece_type, m.from, m.to
        );
    } else {
        println!(
            "Invalid move: {:?} from index {} to index {}",
            piece_type, m.from, m.to
        );
    }

    valid_move
}

// By the time this is called we know the from location is valid
// TODO: implement en passant
fn validate_pawn_move(board: &Board, m: &Move) -> bool {
    let valid_to_location = validate_to_location(board, m);

    if !valid_to_location {
        return false;
    }

    // check to see if the pawn moving to a valid location
    let direction = match board.active_color {
        Color::White => 1,
        Color::Black => -1,
    };

    let from_rank = m.from / 8;
    let to_rank = m.to / 8;

    let from_file = m.from % 8;
    let to_file = m.to % 8;

    // Check if the pawn is moving forward one or two squares
    let rank_diff = (to_rank as i8 - from_rank as i8);

    if rank_diff != direction && rank_diff != 2 * direction {
        println!(
            "Invalid move: Pawn moving in the wrong direction, {} -> {}",
            from_rank + 1,
            to_rank + 1
        );
        return false;
    }

    // Check if the pawn is moving diagonally
    if (to_file != from_file) {
        return false;
    }

    // Check if the pawn is moving two squares forward
    if (to_rank as i8 - from_rank as i8).abs() == 2 {
        if (from_rank != 1 && board.active_color == Color::White)
            || (from_rank != 6 && board.active_color == Color::Black)
        {
            println!(
                "Invalid move: Pawn moving two squares forward from non starting rank {}",
                from_rank + 1
            );
            return false;
        }
    }

    true
}

fn validate_knight_move(board: &Board, m: &Move) -> bool {
    true
}
fn validate_bishop_move(board: &Board, m: &Move) -> bool {
    true
}
fn validate_rook_move(board: &Board, m: &Move) -> bool {
    true
}
fn validate_queen_move(board: &Board, m: &Move) -> bool {
    true
}
fn validate_king_move(board: &Board, m: &Move) -> bool {
    true
}

fn find_peice_at_from_location(board: &Board, m: &Move) -> Option<PieceType> {
    // Obtain a slice of bitboards based on the active color
    let bitboards: &[u64] = match board.active_color {
        Color::White => &board.bitboards[0..6], // First 6 bitboards for White
        Color::Black => &board.bitboards[6..12], // Last 6 bitboards for Black
    };

    // dbg!("{}", color_to_move);

    // Create a bitmask for the 'from' square
    let from_bit = 1u64 << m.from;

    // Find the index of the bitboard that has the 'from' bit set
    let piecetype_at_from_idx = bitboards.iter().position(|&bb| bb & from_bit != 0);

    dbg!("{:?}", piecetype_at_from_idx);
    // If no piece is found at the 'from' square, the move is invalid
    let piece_type = match piecetype_at_from_idx {
        Some(idx) => match board.active_color {
            Color::White => match idx {
                0 => PieceType::Pawn,
                1 => PieceType::Knight,
                2 => PieceType::Bishop,
                3 => PieceType::Rook,
                4 => PieceType::Queen,
                5 => PieceType::King,
                _ => return None, // Invalid index
            },
            Color::Black => match idx {
                0 => PieceType::Pawn,
                1 => PieceType::Knight,
                2 => PieceType::Bishop,
                3 => PieceType::Rook,
                4 => PieceType::Queen,
                5 => PieceType::King,
                _ => return None, // Invalid index
            },
        },
        None => {
            println!("No piece found at 'from'");
            return None;
        } // No piece found at 'from'
    };

    Some(piece_type)
}

fn validate_to_location(board: &Board, m: &Move) -> bool {
    let to_bit = 1u64 << m.to;

    // first check if the 'to' square is occupied by a non capturable piece (e.g. king + friendly piece)
    let friendly_bitboards: &[u64] = match board.active_color {
        Color::White => &board.bitboards[0..6], // First 6 bitboards for White
        Color::Black => &board.bitboards[6..12], // Last 6 bitboards for Black
    };

    let friendly_piece_at_to = friendly_bitboards.iter().any(|&bb| bb & to_bit != 0);

    let enemy_king_bitboard = match board.active_color {
        Color::White => board.bitboards[11], // Black king
        Color::Black => board.bitboards[5],  // White king
    };

    let enemy_king_at_to = enemy_king_bitboard & to_bit != 0;

    if friendly_piece_at_to {
        println!("Attempting to capture friendly piece at '{}'", m.to);
    } else if enemy_king_at_to {
        println!("Attempting to capture enemy king at '{}'", m.to);
    }

    return (!friendly_piece_at_to) && (!enemy_king_at_to);
}
