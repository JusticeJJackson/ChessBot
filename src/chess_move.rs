use crate::board::Board;
use crate::board::Color;
use crate::board::PieceType;
use crate::utils::convert_board_coordinate_to_idx;

// Uses UCI Notation
#[derive(Debug, Clone)]
pub struct Move {
    pub from: u8,                     // The index of the square the piece is moving from
    pub to: u8,                       // The index of the square the piece is moving to
    pub promotion: Option<PieceType>, // Use the Piece struct here
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
        println!("Invalid move: Pawn moving to an invalid location {}", m.to);
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

    // Check if the pawn is moving diagonally more than one square
    if (to_file as i8 - from_file as i8).abs() > 1 {
        println!("Invalid move: Pawn moving diagonally more than one square");
        return false;
    }

    // If pawn is moving diagonally, it must be capturing an enemy piece
    if (to_file as i8 - from_file as i8).abs() > 1 {
        let to_bit = 1u64 << m.to;
        let bitboards = match board.active_color {
            Color::White => &board.bitboards[6..11], // Last 5 bitboards for Black (excluding King)
            Color::Black => &board.bitboards[0..5],  // First 5 bitboards for White (excluding King)
        };

        let enemy_piece_at_to = bitboards.iter().any(|&bb| bb & to_bit != 0);

        if !enemy_piece_at_to {
            println!("Invalid move: Pawn moving diagonally without capturing");
            return false;
        }
    }

    // Check if the pawn is moving two squares forward when its not in starting rank
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

    // Check if pawn is moving two squares forward and the square in front is occupied
    if (to_rank as i8 - from_rank as i8).abs() == 2 {
        if (from_rank != 1 && board.active_color == Color::White)
            || (from_rank != 6 && board.active_color == Color::Black)
        {
            let square_in_front = match board.active_color {
                Color::White => m.from + 8,
                Color::Black => m.from - 8,
            };

            let square_in_front_bit = 1u64 << square_in_front;

            let square_in_front_occupied = board
                .bitboards
                .iter()
                .any(|&bb| bb & square_in_front_bit != 0);

            if square_in_front_occupied {
                println!(
                    "Invalid move: Pawn moving two squares forward when square in front is occupied"
                );
                return false;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Initializes a standard chess board for testing.
    fn setup_standard_board() -> Board {
        // Standard starting position
        Board::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    /// Initializes a custom chess board from a given FEN string.
    fn setup_custom_board(fen: &str) -> Board {
        Board::fen_to_board(fen)
    }

    /// Helper function to create a move and validate it.
    fn validate_move_helper(fen: &str, move_str: &str, expected: bool) -> bool {
        let board = setup_custom_board(fen);
        let m = Move::new(move_str.to_string());
        let valid = validate_move(&board, &m);
        assert_eq!(
            valid,
            expected,
            "Move {} on board {} should be {}",
            move_str,
            fen,
            if expected { "valid" } else { "invalid" }
        );
        valid
    }

    /// Test: Pawn moves one square forward from the initial position.
    #[test]
    fn test_pawn_move_forward_one() {
        let board = setup_custom_board("8/8/8/8/8/8/4P3/8 w - - 0 1");
        let m = Move::new("e2e3".to_string());
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn move from e2 to e3 should be valid");
    }

    /// Test: Pawn moves two squares forward from the initial position.
    #[test]
    fn test_pawn_move_forward_two() {
        let board = setup_standard_board();
        let m = Move::new("e2e4".to_string());
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn move from e2 to e4 should be valid");
    }

    /// Test: Pawn attempts to move three squares forward (invalid).
    #[test]
    fn test_pawn_move_forward_three_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2e5".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to e5 should be invalid");
    }

    /// Test: Pawn attempts to move backward (invalid).
    #[test]
    fn test_pawn_move_backward_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2e1".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to e1 should be invalid");
    }

    /// Test: Pawn attempts to move sideways (invalid).
    #[test]
    fn test_pawn_move_sideways_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2d2".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to d2 should be invalid");
    }

    /// Test: Pawn captures diagonally to the left.
    #[test]
    fn test_pawn_capture_left() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let m = Move::new("d4c5".to_string()); // White pawn on d4 captures to c5
        let valid = validate_move_helper(fen, "d4c5", true);
        assert!(valid, "Pawn capture from d4 to c5 should be valid");
    }

    /// Test: Pawn captures diagonally to the right.
    #[test]
    fn test_pawn_capture_right() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let m = Move::new("d4e5".to_string()); // White pawn on d4 captures to e5
        let valid = validate_move_helper(fen, "d4e5", true);
        assert!(valid, "Pawn capture from d4 to e5 should be valid");
    }

    /// Test: Pawn attempts to capture without an opponent's piece (invalid).
    #[test]
    fn test_pawn_capture_no_piece_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
        let m = Move::new("d4e5".to_string()); // White pawn on d4 attempts to capture to e5, but e5 is empty
        let valid = validate_move_helper(fen, "d4e5", false);
        assert!(
            !valid,
            "Pawn capture from d4 to e5 should be invalid as no piece is present"
        );
    }

    /// Test: Pawn move blocked by another piece.
    #[test]
    fn test_pawn_move_blocked() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let m = Move::new("e3e4".to_string()); // Black pawn attempts to move from e3 to e4, but e4 is blocked by White pawn
        let valid = validate_move_helper(fen, "e3e4", false);
        assert!(
            !valid,
            "Pawn move from e3 to e4 should be invalid because e4 is occupied"
        );
    }

    /// Test: En Passant capture (White captures Black pawn).
    #[test]
    fn test_pawn_en_passant_capture_white() {
        let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq e6 0 3";
        let m = Move::new("d5e4".to_string()); // Black pawn on d5 captures White pawn on e5 via En Passant
        let valid = validate_move_helper(fen, "d5e4", true);
        assert!(
            valid,
            "Black pawn performs En Passant capture from d5 to e4 should be valid"
        );
    }

    /// Test: En Passant capture (Black captures White pawn).
    #[test]
    fn test_pawn_en_passant_capture_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/4pP2/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3";
        let m = Move::new("f4e5".to_string()); // White pawn on f4 captures Black pawn on e5 via En Passant
        let valid = validate_move_helper(fen, "f4e5", true);
        assert!(
            valid,
            "White pawn performs En Passant capture from f4 to e5 should be valid"
        );
    }

    /// Test: En Passant capture attempt when not possible (invalid).
    #[test]
    fn test_pawn_en_passant_invalid() {
        let fen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let m = Move::new("d4c5".to_string()); // Attempting En Passant without the necessary conditions
        let valid = validate_move_helper(fen, "d4c5", false);
        assert!(
            !valid,
            "Pawn En Passant capture from d4 to c5 should be invalid as conditions are not met"
        );
    }

    /// Test: Pawn promotion without reaching the last rank (invalid).
    #[test]
    fn test_pawn_promotion_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let m = Move::new("e2e3".to_string()); // Pawn moves forward without promoting
        let valid = validate_move_helper(fen, "e2e3", true);
        assert!(
            valid,
            "Pawn move from e2 to e3 should be valid and not a promotion"
        );
    }

    /// Test: Pawn promotion upon reaching the last rank.
    #[test]
    fn test_pawn_promotion_valid() {
        let fen = "8/P7/8/8/8/8/8/8 w - - 0 1"; // White pawn on a7
        let m = Move::new("a7a8Q".to_string()); // Promote to Queen
        let valid = validate_move_helper(fen, "a7a8", true);
        assert!(valid, "Pawn promotion from a7 to a8 should be valid");
        // Additional checks can be added to verify the promotion piece if implemented
    }

    /// Test: Pawn attempts to promote without reaching the last rank (invalid).
    #[test]
    fn test_pawn_promotion_without_reaching_last_rank_invalid() {
        let fen = "8/P7/8/8/8/8/8/8 w - - 0 1"; // White pawn on a7
        let m = Move::new("a7a7Q".to_string()); // Invalid promotion (no movement)
        let valid = validate_move_helper(fen, "a7a7", false);
        assert!(!valid, "Pawn promotion without movement should be invalid");
    }

    /// Test: Pawn move at the edge of the board (file 'a').
    #[test]
    fn test_pawn_move_edge_file_a() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b KQkq - 0 1"; // White pawn on a2
        let m = Move::new("a2a4".to_string()); // Move two squares forward
        let valid = validate_move_helper(fen, "a2a4", true);
        assert!(valid, "Pawn move from a2 to a4 on file 'a' should be valid");
    }

    /// Test: Pawn move at the edge of the board (file 'h').
    #[test]
    fn test_pawn_move_edge_file_h() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 w KQkq - 0 1"; // White pawn on h2
        let m = Move::new("h2h4".to_string()); // Move two squares forward
        let valid = validate_move_helper(fen, "h2h4", true);
        assert!(valid, "Pawn move from h2 to h4 on file 'h' should be valid");
    }

    /// Test: Pawn attempts to promote but does not reach the last rank (invalid).
    #[test]
    fn test_pawn_promotion_not_reaching_last_rank_invalid() {
        let fen = "8/P7/8/8/8/8/8/8 w - - 0 1"; // White pawn on a7
        let m = Move::new("a7a7".to_string()); // No movement, invalid promotion
        let valid = validate_move_helper(fen, "a7a7", false);
        assert!(
            !valid,
            "Pawn promotion attempt without movement should be invalid"
        );
    }

    /// Test: Pawn reaches the last rank and promotes to Queen.
    #[test]
    fn test_pawn_promotion_to_queen() {
        let fen = "8/P7/8/8/8/8/8/8 w - - 0 1"; // White pawn on a7
        let m = Move::new("a7a8Q".to_string()); // Promote to Queen
        let board = setup_custom_board("8/P7/8/8/8/8/8/8 w - - 0 1");
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn promotion from a7 to a8 should be valid");
        // Additional checks can be implemented to verify the promoted piece
    }

    /// Test: Pawn attempts to capture the enemy king (invalid).
    #[test]
    fn test_pawn_capture_enemy_king_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"; // Black king at e8
        let m = Move::new("e7e8".to_string()); // Black pawn attempts to move to e8 where the king is
        let valid = validate_move_helper(fen, "e7e8", false);
        assert!(
            !valid,
            "Pawn capture of enemy king from e7 to e8 should be invalid"
        );
    }

    /// Test: Pawn captures enemy piece directly in front (invalid, since pawns cannot capture forward).
    #[test]
    fn test_pawn_capture_forward_invalid() {
        let fen = "rnbqkbnr/pppp1ppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; // Black pawn on e4
        let m = Move::new("d2e3".to_string()); // White pawn on d2 attempts to capture forward to e3, but e3 is empty
        let valid = validate_move_helper(fen, "d2e3", false);
        assert!(
            !valid,
            "Pawn capture from d2 to e3 should be invalid as pawns cannot capture forward"
        );
    }

    /// Test: Pawn attempts to move to a square occupied by a friendly piece (invalid).
    #[test]
    fn test_pawn_move_to_friendly_occupied_square_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawns on d4 and e2
        let m = Move::new("d4d5".to_string()); // White pawn on d4 attempts to move to d5, which is empty
        let valid = validate_move_helper(fen, "d4d5", true);
        assert!(
            valid,
            "Pawn move from d4 to d5 should be valid as the square is empty"
        );

        let fen_friendly = "rnbqkbnr/pppppppp/8/8/3P4/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawns on d4 and d3
        let m_friendly = Move::new("d4d3".to_string()); // White pawn on d4 attempts to move backward to d3 (invalid)
        let valid_friendly = validate_move_helper(fen_friendly, "d4d3", false);
        assert!(
            !valid_friendly,
            "Pawn move from d4 to d3 should be invalid as pawns cannot move backward"
        );
    }

    /// Test: Pawn attempts to move two squares forward from a non-initial rank (invalid).
    #[test]
    fn test_pawn_move_two_squares_non_initial_rank_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawn on d4
        let m = Move::new("d4d6".to_string()); // White pawn attempts to move two squares forward from d4 to d6
        let valid = validate_move_helper(fen, "d4d6", false);
        assert!(
            !valid,
            "Pawn move from d4 to d6 should be invalid as it's not the initial position"
        );
    }

    /// Test: Pawn attempts to move two squares forward from the initial rank when blocked (invalid).
    #[test]
    fn test_pawn_move_two_squares_blocked_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1"; // White pawn on e3
        let m = Move::new("e3e5".to_string()); // White pawn attempts to move two squares forward from e3 to e5
        let valid = validate_move_helper(fen, "e3e5", false);
        assert!(
            !valid,
            "Pawn move from e3 to e5 should be invalid as the path is blocked"
        );
    }
}
