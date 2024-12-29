use crate::board::Board;
use crate::board::Color;
use crate::board::PieceType;
use crate::utils::convert_board_coordinate_to_idx;
use crate::utils::EDGE_DISTANCES;
use std::collections::HashSet;
use std::hash::Hash;

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

    // Ensure that peice is not promoting if its not a pawn
    if piece_type != PieceType::Pawn {
        if m.promotion.is_some() {
            println!("Invalid move: Non-pawn piece attempting to promote");
            return false;
        }
    }

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
    if (to_file as i8 - from_file as i8).abs() == 1 {
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

    // Check if the pawn is moving to the last rank
    if to_rank == 0 || to_rank == 7 {
        if m.promotion.is_none() {
            println!("Invalid move: Pawn moving to last rank without promotion");
            return false;
        } else {
            // Check to see if promotion peice type is valid
            let promotion_piece = m.promotion.unwrap();

            if promotion_piece == PieceType::Pawn || promotion_piece == PieceType::King {
                println!("Invalid move: Pawn promotion to invalid piece type");
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
    // Ensures we are not capturing a friendly piece or the enemy king
    let valid_to_location = validate_to_location(board, m);

    if !valid_to_location {
        println!(
            "Invalid move: Bishop moving to an invalid location {}",
            m.to
        );
        return false;
    }

    let moves = generate_sliding_moves(board, PieceType::Bishop, m);

    if !moves.contains(&m.to) {
        println!(
            "Invalid move: Bishop moving to an invalid location {}",
            m.to
        );
        return false;
    }

    true
}
fn validate_rook_move(board: &Board, m: &Move) -> bool {
    // Ensures we are not capturing a friendly piece or the enemy king
    let valid_to_location = validate_to_location(board, m);

    if !valid_to_location {
        println!("Invalid move: Rook moving to an invalid location {}", m.to);
        return false;
    }

    let moves = generate_sliding_moves(board, PieceType::Rook, m);

    if !moves.contains(&m.to) {
        println!(
            "Invalid move: Bishop moving to an invalid location {}",
            m.to
        );
        return false;
    }

    true
}
fn validate_queen_move(board: &Board, m: &Move) -> bool {
    // Ensures we are not capturing a friendly piece or the enemy king
    let valid_to_location = validate_to_location(board, m);

    if !valid_to_location {
        println!("Invalid move: Queen moving to an invalid location {}", m.to);
        return false;
    }

    let moves = generate_sliding_moves(board, PieceType::Queen, m);

    if !moves.contains(&m.to) {
        println!("Invalid move: Queen moving to an invalid location {}", m.to);
        return false;
    }

    true
}
fn validate_king_move(board: &Board, m: &Move) -> bool {
    true
}

pub fn generate_sliding_moves(board: &Board, piece_type: PieceType, m: &Move) -> HashSet<u8> {
    let mut moves = HashSet::new();

    let capturable_bitboards: &[u64] = match board.active_color {
        Color::White => &board.bitboards[6..11], // Last 5 bitboards for Black (Excluding King)
        Color::Black => &board.bitboards[0..5],  // First 5 bitboards for White (Excluding King)
    };

    let friendly_bitboards: &[u64] = match board.active_color {
        Color::White => &board.bitboards[0..6], // First 6 bitboards for White
        Color::Black => &board.bitboards[6..12], // Last 6 bitboards for Black
    };

    let enemy_king_bitboard = match board.active_color {
        Color::White => board.bitboards[11], // Black king
        Color::Black => board.bitboards[5],  // White king
    };

    match piece_type {
        PieceType::Bishop => {
            let distance_to_jump: [i8; 4] = [9, 7, -7, -9]; // [NE, NW, SE, SW]
            let dir: [u8; 4] = [4, 5, 6, 7];

            // for each direction [NE, NW, SE, SW]
            for i in 0..4 {
                // get the max distance to the edge of the board for that given direction
                let max_distance = EDGE_DISTANCES[dir[i] as usize][m.from as usize];

                // for each square in that direction jumping by the distance to the edge
                for hop_distance_multiplier in 1..=max_distance {
                    let hop_distance = distance_to_jump[i] * hop_distance_multiplier as i8;

                    let to: u8 = ((m.from as i8) + hop_distance) as u8;

                    // dbg!("{:?}", to);
                    let to_bit: u64 = 1u64 << to;
                    // if the square is occupied by a friendly piece or the enemy king, stop
                    if friendly_bitboards.iter().any(|&bb| bb & to_bit != 0)
                        || enemy_king_bitboard & to_bit != 0
                    {
                        break;
                    }

                    // if the square is occupied by an enemy piece, add it to the moves set and stop
                    if capturable_bitboards.iter().any(|&bb| bb & to_bit != 0) {
                        moves.insert(to);
                        break;
                    }

                    // if the square is empty, add it to the moves set
                    moves.insert(to);
                }
            }
        }
        PieceType::Rook => {
            // Define the distance offsets for Rook movement:
            // [North, South, East, West]
            let distance_to_jump: [i8; 4] = [8, -8, 1, -1];

            // Match these directions to EDGE_DISTANCES indices:
            // 0 = North, 1 = South, 2 = East, 3 = West
            let dir: [u8; 4] = [0, 1, 2, 3];

            // For each direction, move along that line until blocked or edge is reached.
            for i in 0..4 {
                // Get max squares available in this direction from the current square
                let max_distance = EDGE_DISTANCES[dir[i] as usize][m.from as usize];

                // Move up to `max_distance` squares in this direction
                for hop_distance_multiplier in 1..=max_distance {
                    let hop_distance = distance_to_jump[i] * hop_distance_multiplier as i8;

                    let to: i8 = m.from as i8 + hop_distance;
                    // If we go out of the 0..63 range, stop
                    if to < 0 || to >= 64 {
                        break;
                    }

                    let to_u8: u8 = to as u8;
                    let to_bit: u64 = 1u64 << to_u8;

                    // If friendly piece or enemy king occupies this square, stop.
                    if friendly_bitboards.iter().any(|&bb| bb & to_bit != 0)
                        || enemy_king_bitboard & to_bit != 0
                    {
                        break;
                    }

                    // dbg!("{:?}", to_u8);

                    // If an enemy piece occupies this square, add it as a capture and stop.
                    if capturable_bitboards.iter().any(|&bb| bb & to_bit != 0) {
                        moves.insert(to_u8);
                        break;
                    }

                    // If it's empty, add this square as a valid move and continue.
                    moves.insert(to_u8);
                }
            }
        }
        PieceType::Queen => {
            // Combine Rook and Bishop moves for the Queen
            moves.extend(generate_sliding_moves(board, PieceType::Rook, m));
            moves.extend(generate_sliding_moves(board, PieceType::Bishop, m));
        }
        _ => {
            println!("Invalid piece type for sliding move generation");
        }
    }
    moves
}
pub fn find_peice_at_from_location(board: &Board, m: &Move) -> Option<PieceType> {
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
    fn test_validitiy_of_pawn_move_forward_one() {
        let board = setup_custom_board("8/8/8/8/8/8/4P3/8 w - - 0 1");
        let m = Move::new("e2e3".to_string());
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn move from e2 to e3 should be valid");
    }

    /// Test: Pawn moves two squares forward from the initial position.
    #[test]
    fn test_validitiy_of_pawn_move_forward_two() {
        let board = setup_standard_board();
        let m = Move::new("e2e4".to_string());
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn move from e2 to e4 should be valid");
    }

    /// Test: Pawn attempts to move three squares forward (invalid).
    #[test]
    fn test_validitiy_of_pawn_move_forward_three_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2e5".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to e5 should be invalid");
    }

    /// Test: Pawn attempts to move backward (invalid).
    #[test]
    fn test_validitiy_of_pawn_move_backward_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2e1".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to e1 should be invalid");
    }

    /// Test: Pawn attempts to move sideways (invalid).
    #[test]
    fn test_validitiy_of_pawn_move_sideways_invalid() {
        let board = setup_standard_board();
        let m = Move::new("e2d2".to_string());
        let valid = validate_move(&board, &m);
        assert!(!valid, "Pawn move from e2 to d2 should be invalid");
    }

    /// Test: Pawn captures diagonally to the left.
    #[test]
    fn test_validitiy_of_pawn_capture_left() {
        let fen = "8/8/8/8/8/5p2/4P3/8 w - - 0 1";
        let valid = validate_move_helper(fen, "e2f3", true); // White pawn on e2 captures Black pawn on f3
        assert!(valid, "Pawn capture from d4 to c5 should be valid");
    }

    /// Test: Pawn captures diagonally to the right.
    #[test]
    fn test_validitiy_of_pawn_capture_right() {
        let fen = "8/8/8/8/8/3p4/4P3/8 w - - 0 1";
        let valid = validate_move_helper(fen, "e2d3", true); // White pawn on e2 captures Black pawn on d3
        assert!(valid, "Pawn capture from d4 to e5 should be valid");
    }

    /// Test: Pawn attempts to capture without an opponent's piece (invalid).
    #[test]
    fn test_validitiy_of_pawn_capture_no_piece_invalid() {
        let fen = "rnbqkbnr/pppp1ppp/8/8/8/4p3/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let valid = validate_move_helper(fen, "e2d3", false);
        assert!(
            !valid,
            "Pawn capture from d4 to e5 should be invalid as no piece is present"
        );
    }

    /// Test: Pawn move blocked by another piece.
    #[test]
    fn test_validitiy_of_pawn_move_blocked() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let m = Move::new("e3e4".to_string()); // Black pawn attempts to move from e3 to e4, but e4 is blocked by White pawn
        let valid = validate_move_helper(fen, "e3e4", false);
        assert!(
            !valid,
            "Pawn move from e3 to e4 should be invalid because e4 is occupied"
        );
    }

    /// Test: En Passant capture (White captures Black pawn).
    /// TODO: Implement En Passant capture

    // #[test]
    // fn test_validitiy_of_pawn_en_passant_capture_white() {
    //     let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq e6 0 3";
    //     let m = Move::new("d5e4".to_string()); // Black pawn on d5 captures White pawn on e5 via En Passant
    //     let valid = validate_move_helper(fen, "d5e4", true);
    //     assert!(
    //         valid,
    //         "Black pawn performs En Passant capture from d5 to e4 should be valid"
    //     );
    // }

    /// Test: En Passant capture (Black captures White pawn).

    // #[test]
    // fn test_validitiy_of_pawn_en_passant_capture_black() {
    //     let fen = "rnbqkbnr/pppppppp/8/8/4pP2/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3";
    //     let m = Move::new("f4e5".to_string()); // White pawn on f4 captures Black pawn on e5 via En Passant
    //     let valid = validate_move_helper(fen, "f4e5", true);
    //     assert!(
    //         valid,
    //         "White pawn performs En Passant capture from f4 to e5 should be valid"
    //     );
    // }

    /// Test: En Passant capture attempt when not possible (invalid).

    // #[test]
    // fn test_validitiy_of_pawn_en_passant_invalid() {
    //     let fen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    //     let m = Move::new("d4c5".to_string()); // Attempting En Passant without the necessary conditions
    //     let valid = validate_move_helper(fen, "d4c5", false);
    //     assert!(
    //         !valid,
    //         "Pawn En Passant capture from d4 to c5 should be invalid as conditions are not met"
    //     );
    // }

    // TODO: Add Test for for promotion

    #[test]
    fn test_validity_of_white_pawn_promotion_to_queen() {
        // White pawn on h7, ready to promote
        let fen = "8/7P/8/8/8/8/8/8 w - - 0 1";
        // White to move, h7 to h8 (queen promotion)
        let valid = validate_move_helper(fen, "h7h8q", true);
        assert!(
            valid,
            "Pawn promotion h7h8q should be valid for a white pawn promoting to a queen."
        );
    }

    #[test]
    fn test_validity_of_black_pawn_promotion_to_queen() {
        // Black pawn on a2, ready to promote (note that it's Black to move)
        let fen = "8/8/8/8/8/8/p7/7K b - - 0 1";
        // Black to move, a2 to a1 (queen promotion)
        let valid = validate_move_helper(fen, "a2a1q", true);
        assert!(
            valid,
            "Pawn promotion a2a1q should be valid for a black pawn promoting to a queen."
        );
    }

    #[test]
    fn test_invalidity_of_pawn_promotion_on_seventh_rank() {
        // White pawn on h6, not on the last rank
        let fen = "8/8/8/8/8/7P/8/7K w - - 0 1";
        // White tries to promote from h6 to h7 (incorrect rank for promotion)
        let valid = validate_move_helper(fen, "h6h7q", false);
        assert!(
            !valid,
            "Pawn promotion from h6 to h7 should be invalid since it's not the last rank."
        );
    }

    #[test]
    fn test_invalidity_of_pawn_promoting_to_king() {
        // White pawn on h7, ready to promote
        let fen = "8/8/8/8/8/8/7P/7K w - - 0 1";
        // White tries to promote to a king (which should be invalid)
        let valid = validate_move_helper(fen, "h7h8k", false);
        assert!(
            !valid,
            "Pawn promotion h7h8k should be invalid, as promoting to a king is not allowed."
        );
    }

    #[test]
    fn test_invalidity_of_pawn_promoting_to_pawn() {
        // White pawn on h7, ready to promote
        let fen = "8/8/8/8/8/8/7P/7K w - - 0 1";
        // White tries to promote to another pawn (which should be invalid)
        let valid = validate_move_helper(fen, "h7h8p", false);
        assert!(
            !valid,
            "Pawn promotion h7h8p should be invalid, as promoting to a pawn is not allowed."
        );
    }

    #[test]
    fn test_validity_of_white_pawn_promotion_to_rook() {
        // White pawn on e7, ready to promote
        let fen = "8/4P3/8/8/8/8/8/8 w - - 0 1";
        // White to move, e7 to e8 (rook promotion)
        let valid = validate_move_helper(fen, "e7e8r", true);
        assert!(
            valid,
            "Pawn promotion e7e8r should be valid for a white pawn promoting to a rook."
        );
    }

    #[test]
    fn test_validity_of_white_pawn_promotion_to_knight() {
        // White pawn on b7, ready to promote
        let fen = "8/1P6/8/8/8/8/8/8 w - - 0 1";
        // White to move, b7 to b8 (knight promotion)
        let valid = validate_move_helper(fen, "b7b8n", true);
        assert!(
            valid,
            "Pawn promotion b7b8n should be valid for a white pawn promoting to a knight."
        );
    }

    #[test]
    fn test_validity_of_white_pawn_promotion_to_bishop() {
        // White pawn on c7, ready to promote
        let fen = "8/2P5/8/8/8/8/8/8 w - - 0 1";
        // White to move, c7 to c8 (bishop promotion)
        let valid = validate_move_helper(fen, "c7c8b", true);
        assert!(
            valid,
            "Pawn promotion c7c8b should be valid for a white pawn promoting to a bishop."
        );
    }

    /// Test: Pawn move at the edge of the board (file 'a').
    #[test]
    fn test_validitiy_of_pawn_move_edge_file_a() {
        let board = setup_standard_board();
        let m = Move::new("a2a4".to_string()); // Move two squares forward
        let valid = validate_move(&board, &m);
        assert!(valid, "Pawn move from a2 to a4 on file 'a' should be valid");
    }

    /// Test: Pawn move at the edge of the board (file 'h').
    #[test]
    fn test_validitiy_of_pawn_move_edge_file_h() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 w KQkq - 0 1"; // White pawn on h2
        let m = Move::new("h2h4".to_string()); // Move two squares forward
        let valid = validate_move_helper(fen, "h2h4", true);
        assert!(valid, "Pawn move from h2 to h4 on file 'h' should be valid");
    }

    /// Test: Pawn captures enemy piece directly in front (invalid, since pawns cannot capture forward).
    #[test]
    fn test_validitiy_of_pawn_capture_forward_invalid() {
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
    fn test_validitiy_of_pawn_move_to_friendly_occupied_square_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawns on d4 and e2
        let valid = validate_move_helper(fen, "d4d5", true); // White pawn on d4 attempts to move to d5, which is empty
        assert!(
            valid,
            "Pawn move from d4 to d5 should be valid as the square is empty"
        );

        let fen_friendly = "rnbqkbnr/pppppppp/8/8/3P4/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawns on d4 and d3
        let valid_friendly = validate_move_helper(fen_friendly, "d4d3", false); // White pawn on d4 attempts to move backward to d3 (invalid)
        assert!(
            !valid_friendly,
            "Pawn move from d4 to d3 should be invalid as pawns cannot move backward"
        );
    }

    /// Test: Pawn attempts to move two squares forward from a non-initial rank (invalid).
    #[test]
    fn test_validitiy_of_pawn_move_two_squares_non_initial_rank_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"; // White pawn on d4
        let valid = validate_move_helper(fen, "d4d6", false); // White pawn attempts to move two squares forward from d4 to d6
        assert!(
            !valid,
            "Pawn move from d4 to d6 should be invalid as it's not the initial position"
        );
    }

    /// Test: Pawn attempts to move two squares forward from the initial rank when blocked (invalid).
    #[test]
    fn test_validitiy_of_pawn_move_two_squares_blocked_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1"; // White pawn on e3
        let valid = validate_move_helper(fen, "e3e5", false); // White pawn attempts to move two squares forward from e3 to e5
        assert!(
            !valid,
            "Pawn move from e3 to e5 should be invalid as the path is blocked"
        );
    }

    // Test: Attempt to move White Bishop one square diagonal (valid)
    #[test]
    fn test_validity_of_bishop_move_one_square_diagonal() {
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1"; // White bishop on d4
        let valid = validate_move_helper(fen, "d4e5", true); // White bishop moves from d4 to e5
        assert!(
            valid,
            "Bishop move from d4 to e5 should be valid as it's one square diagonal"
        );
    }

    // Test: Attempt to move White Bishop to the corner diagonal (valid)
    #[test]
    fn test_validity_of_bishop_move_to_corner_diagonal() {
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1"; // White bishop on d4
        let valid = validate_move_helper(fen, "d4h8", true); // White bishop moves from d4 to e5
        assert!(
            valid,
            "Bishop move from d4 to h8 should be valid as it's 4 square diagonal"
        );
    }

    // Test: Attempt to move White Bishop to the corner diagonal when there is a queen there (invalid)
    #[test]
    fn test_invalidity_of_bishop_move_to_corner_diagonal_when_queen_present() {
        let fen = "7Q/8/8/8/3B4/8/8/8 w - - 0 1"; // White bishop on d4
        let valid = validate_move_helper(fen, "d4h8", false); // White bishop moves from d4 to h8
        assert!(
            !valid,
            "Bishop move from d4 to h8 should be invalid as there is a queen there"
        );
    }

    // Test: Attempt to move White Bishop to the corner diagonal when there is a black queen there (valid)
    #[test]
    fn test_validity_of_bishop_move_to_corner_diagonal_when_enemy_queen_present() {
        let fen = "7q/8/8/8/3B4/8/8/8 w - - 0 1"; // White bishop on d4 Black Queen on h8
        let valid = validate_move_helper(fen, "d4h8", true); // White bishop moves from d4 to h8
        assert!(
            valid,
            "Bishop move from d4 to h8 should be invalid as there is a queen there"
        );
    }

    /// Test: Bishop is blocked by a friendly piece on e5, cannot move or capture beyond it.
    #[test]
    fn test_bishop_blocked_by_friendly_piece() {
        // White bishop on d4, White rook on e5, Black rook on f6
        // Board visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . . .
        // 5 | . . . . R . . .
        // 4 | . . . B . . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/8/4R3/3B4/8/8/8 w - - 0 1

        let fen = "8/8/8/4R3/3B4/8/8/8 w - - 0 1";

        // Trying to move d4 -> f6 (jumping over e5) should be invalid
        let valid = validate_move_helper(fen, "d4f6", false);
        assert!(
            !valid,
            "Bishop should not be able to jump over a friendly rook on e5"
        );
    }

    /// Test: Bishop is blocked by an enemy piece on e5, so it cannot capture anything beyond e5.
    #[test]
    fn test_bishop_blocked_by_enemy_piece() {
        // White bishop on d4, Black rook on e5, Black rook on f6
        // Board visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . r .
        // 5 | . . . . r . . .
        // 4 | . . . B . . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/5r2/4r3/3B4/8/8/8 w - - 0 1

        let fen = "8/8/5r2/4r3/3B4/8/8/8 w - - 0 1";

        // Attempt to move d4 -> f6 is invalid because the bishop cannot jump over the rook on e5
        let valid_blocked = validate_move_helper(fen, "d4f6", false);
        assert!(
            !valid_blocked,
            "Bishop cannot jump over an enemy rook on e5 to reach f6"
        );
    }

    /// Test: Bishop captures the enemy piece on e5, but cannot go further to f6 in one move.
    #[test]
    fn test_bishop_capture_enemy_piece_but_not_jump_further() {
        // White bishop on d4, Black rook on e5, another Black rook on f6
        // Board visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . r .
        // 5 | . . . . r . . .
        // 4 | . . . B . . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/5r2/4r3/3B4/8/8/8 w - - 0 1

        let fen = "8/8/5r2/4r3/3B4/8/8/8 w - - 0 1";

        // d4 -> e5 is valid (capture the black rook at e5)
        let valid_capture = validate_move_helper(fen, "d4e5", true);
        assert!(
            valid_capture,
            "Bishop should be able to capture the enemy rook at e5"
        );

        // However, trying to move from d4 -> f6 in a single move is invalid, because e5 is blocked first
        let invalid_jump = validate_move_helper(fen, "d4f6", false);
        assert!(
        !invalid_jump,
        "Bishop cannot skip over the black rook at e5 to capture another piece on f6 in the same move"
    );
    }

    /// Test: Bishop cannot capture the enemy piece on f6 because a friendly piece on e5 is blocking it.
    #[test]
    fn test_bishop_capture_blocked_by_friendly_piece() {
        // White bishop on d4, White pawn on e5, Black rook on f6
        // Board visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . r .
        // 5 | . . . . P . . .
        // 4 | . . . B . . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/5r2/4P3/3B4/8/8/8 w - - 0 1

        let fen = "8/8/5r2/4P3/3B4/8/8/8 w - - 0 1";

        // Attempting to move the bishop from d4 -> f6 is invalid, because the friendly pawn on e5 blocks the diagonal
        let valid = validate_move_helper(fen, "d4f6", false);
        assert!(
            !valid,
            "Bishop cannot capture the rook on f6 because a friendly pawn on e5 blocks the path"
        );
    }

    #[test]
    fn test_validity_of_black_bishop_simple_diagonal_move() {
        // Black bishop on e4 (index 28).
        // FEN visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . . .
        // 5 | . . . . . . . .
        // 4 | . . . . b . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/8/8/4b3/8/8/8 b - - 0 1
        //
        // Black bishop attempts to move e4 -> h7 (a diagonal move).
        // This should be valid if the path is clear.

        let fen = "8/8/8/8/4b3/8/8/8 b - - 0 1";
        // Check that moving e4 -> h7 is valid for a Black bishop
        let valid = validate_move_helper(fen, "e4h7", true);
        assert!(
            valid,
            "Black bishop move from e4 to h7 should be valid along a clear diagonal."
        );
    }

    #[test]
    fn test_validity_of_black_bishop_capture_white_piece() {
        // Black bishop on e4, White rook on h7.
        // FEN visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . R
        // 6 | . . . . . . . .
        // 5 | . . . . . . . .
        // 4 | . . . . b . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/7R/8/8/4b3/8/8/8 b - - 0 1
        //
        // Black bishop attempts to move e4 -> h7 to capture the White rook.

        let fen = "8/7R/8/8/4b3/8/8/8 b - - 0 1";
        // Attempt to move e4 -> h7 (capture)
        let valid = validate_move_helper(fen, "e4h7", true);
        assert!(
            valid,
            "Black bishop should be able to capture the White rook on h7."
        );
    }

    #[test]
    fn test_invalidity_of_black_bishop_blocked_by_friendly_piece() {
        // Black bishop on e4, Black knight on f5, White rook on g6.
        // The bishop should NOT be able to jump over its own knight to capture the rook.
        //
        // FEN visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . . R
        // 5 | . . . . . n . .
        // 4 | . . . . b . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/7R/5n2/4b3/8/8/8 b - - 0 1
        //
        // Black bishop tries e4 -> g6, but is blocked by the Black knight on f5.

        let fen = "8/8/7R/5n2/4b3/8/8/8 b - - 0 1";
        let valid = validate_move_helper(fen, "e4g6", false);
        assert!(
            !valid,
            "Black bishop should not jump over its own knight on f5 to capture the rook on g6."
        );
    }

    #[test]
    fn test_bishop_capture_enemy_piece_but_not_jump_further_black() {
        // Black bishop on d4, White rook on e5, White rook on f6.
        // The bishop can capture the rook at e5 but cannot continue to f6 in one move.
        //
        // Board visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . . R
        // 5 | . . . . R . . .
        // 4 | . . . b . . . .
        // 3 | . . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/7R/4R3/3b4/8/8/8 b - - 0 1

        let fen = "8/8/7R/4R3/3b4/8/8/8 b - - 0 1";

        // 1. d4 -> e5 is valid (capture the White rook at e5)
        let capture_valid = validate_move_helper(fen, "d4e5", true);
        assert!(
            capture_valid,
            "Black bishop should be able to capture the White rook on e5."
        );

        // 2. Trying d4 -> f6 in one move is invalid, since e5 is occupied by a White rook (blocking further movement).
        let jump_invalid = validate_move_helper(fen, "d4f6", false);
        assert!(
        !jump_invalid,
        "Black bishop cannot jump over the White rook on e5 to capture another piece on f6 in one move."
    );
    }

    #[test]
    fn test_validity_of_black_bishop_moving_to_corner_diagonal() {
        // Black bishop on a3 (index 16), aiming to move to f8 (index 63).
        // FEN visualization:
        // 8 | . . . . . . . .
        // 7 | . . . . . . . .
        // 6 | . . . . . . . .
        // 5 | . . . . . . . .
        // 4 | . . . . . . . .
        // 3 | b . . . . . . .
        // 2 | . . . . . . . .
        // 1 | . . . . . . . .
        //     a b c d e f g h
        // FEN: 8/8/8/8/8/b7/8/8 b - - 0 1
        //
        // Black bishop tries to move a3 -> f8 if the path is clear.

        let fen = "8/8/8/8/8/b7/8/8 b - - 0 1";
        let valid = validate_move_helper(fen, "a3f8", true);
        assert!(
            valid,
            "Black bishop from a3 to h8 should be valid along an unobstructed diagonal."
        );
    }

    #[test]
    fn test_validity_of_rook_moving_one_square() {
        let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d5", true);
        assert!(
            valid,
            "Rook move from d4 to d5 should be valid as it's one square forward"
        );
    }

    #[test]
    fn test_validity_of_rook_moving_many_square() {
        let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d8", true);
        assert!(
            valid,
            "Rook move from d4 to d8 should be valid as it's forward"
        );
    }

    #[test]
    fn test_validity_of_rook_moving_vertically_up() {
        // White rook on d4 moving to d7
        let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d7", true);
        assert!(
            valid,
            "Rook should be able to move vertically up from d4 to d7."
        );
    }

    #[test]
    fn test_validity_of_rook_moving_horizontally() {
        // White rook on d4 moving to g4
        let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4g4", true);
        assert!(
            valid,
            "Rook should be able to move horizontally from d4 to g4."
        );
    }

    #[test]
    fn test_invalidity_of_rook_moving_diagonally() {
        // White rook on d4 attempting to move to e5 (diagonal)
        let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4e5", false);
        assert!(
            !valid,
            "Rook moving from d4 to e5 (diagonal) should be invalid."
        );
    }

    #[test]
    fn test_rook_blocked_by_friendly_piece() {
        // White rook on d4, White bishop on d5; rook tries to move to d6
        // The bishop on d5 blocks it from moving further.
        let fen = "8/8/8/3B4/3R4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d6", false);
        assert!(
            !valid,
            "Rook should be blocked by a friendly bishop on d5 and cannot reach d6."
        );
    }

    #[test]
    fn test_rook_blocked_then_capture_enemy_piece() {
        // White rook on d4, Black knight on d5, White tries to move rook to d5 (capture).
        // This should be valid but cannot move further to d6 in the same move.
        let fen = "8/8/8/3n4/3R4/8/8/8 w - - 0 1";

        // Capture on d5
        let capture_valid = validate_move_helper(fen, "d4d5", true);
        assert!(capture_valid, "Rook should capture the black knight on d5.");
    }

    #[test]
    fn test_rook_blocked() {
        let fen = "8/8/8/3n4/3R4/8/8/8 w - - 0 1";
        // Attempt to move over the knight to d6 in a single move
        let jump_invalid = validate_move_helper(fen, "d4d6", false);
        assert!(
            !jump_invalid,
            "Rook cannot jump over an enemy piece at d5 to reach d6 in one move."
        );
    }

    #[test]
    fn test_bishop_cannot_move_straight_north() {
        // White bishop on d4 trying to move to d5 (north)
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d5", false);
        assert!(
            !valid,
            "Bishop moving straight north (d4 to d5) should be invalid."
        );
    }

    #[test]
    fn test_bishop_cannot_move_straight_south() {
        // White bishop on d4 trying to move to d3 (south)
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d3", false);
        assert!(
            !valid,
            "Bishop moving straight south (d4 to d3) should be invalid."
        );
    }

    #[test]
    fn test_bishop_cannot_move_straight_east() {
        // White bishop on d4 trying to move to e4 (east)
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4e4", false);
        assert!(
            !valid,
            "Bishop moving straight east (d4 to e4) should be invalid."
        );
    }

    #[test]
    fn test_bishop_cannot_move_straight_west() {
        // White bishop on d4 trying to move to c4 (west)
        let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4c4", false);
        assert!(
            !valid,
            "Bishop moving straight west (d4 to c4) should be invalid."
        );
    }

    #[test]
    fn test_moving_queen_horizontally() {
        // White queen on d4 moving to h4
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4h4", true);
        assert!(
            valid,
            "Queen should be able to move horizontally from d4 to h4."
        );
    }

    #[test]
    fn test_moving_queen_vertically() {
        // White queen on d4 moving to d8
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d8", true);
        assert!(
            valid,
            "Queen should be able to move vertically from d4 to d8."
        );
    }

    #[test]
    fn test_validity_of_queen_move_one_square_right() {
        // White queen on d4 moving one square horizontally to e4
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4e4", true);
        assert!(
            valid,
            "Queen should be able to move one square horizontally from d4 to e4."
        );
    }

    #[test]
    fn test_validity_of_queen_move_multiple_squares_vertically() {
        // White queen on d4 moving multiple squares up to d8
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d8", true);
        assert!(
            valid,
            "Queen should be able to move multiple squares vertically from d4 to d8."
        );
    }

    #[test]
    fn test_validity_of_queen_move_diagonal() {
        // White queen on d4 moving diagonally to h8
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4h8", true);
        assert!(
            valid,
            "Queen should be able to move diagonally from d4 to h8."
        );
    }

    #[test]
    fn test_invalidity_of_queen_moving_like_knight() {
        // White queen on d4 attempting a knight-like move to e6
        let fen = "8/8/8/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4e6", false);
        assert!(
            !valid,
            "Queen should not be able to move like a knight from d4 to e6."
        );
    }

    #[test]
    fn test_queen_blocked_by_friendly_piece() {
        // White queen on d4, White pawn on d5; Queen cannot move to d6 in one move.
        // The pawn on d5 blocks the Queen from moving further upward.
        let fen = "8/8/8/3P4/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4d6", false);
        assert!(
            !valid,
            "Queen should be blocked by a friendly pawn on d5 and cannot reach d6."
        );
    }

    #[test]
    fn test_queen_blocked_by_enemy_piece_cannot_jump_over() {
        // White queen on d4, Black rook on d5, Black rook on d6
        // The Queen can capture the rook on d5, but cannot jump over it to reach d6 in one move.
        let fen = "8/8/8/3r4/3Q4/8/8/8 w - - 0 1";

        // Capture on d5
        let capture_valid = validate_move_helper(fen, "d4d5", true);
        assert!(
            capture_valid,
            "Queen should be able to capture the black rook on d5."
        );

        // Attempt to move over the rook to d6 in a single move
        let jump_invalid = validate_move_helper(fen, "d4d6", false);
        assert!(
            !jump_invalid,
            "Queen cannot jump over an enemy piece on d5 to move to d6 in one move."
        );
    }

    #[test]
    fn test_validity_of_queen_capturing_diagonally() {
        // White queen on d4, Black rook on f6
        // The Queen should be able to move diagonally from d4 to f6, capturing the rook.
        let fen = "8/8/8/5r2/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4f6", true);
        assert!(
            valid,
            "Queen should be able to capture the black rook on f6 diagonally."
        );
    }

    #[test]
    fn test_invalidity_of_queen_capture_own_piece() {
        // White queen on d4, White bishop on f6
        // The Queen cannot capture its own bishop, so d4 -> f6 is invalid.
        let fen = "8/8/5B2/8/3Q4/8/8/8 w - - 0 1";
        let valid = validate_move_helper(fen, "d4f6", false);
        assert!(
            !valid,
            "Queen should not be able to capture its own bishop on f6."
        );
    }
}
