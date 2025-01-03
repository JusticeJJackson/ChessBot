use crate::chess_move::{find_peice_at_from_location, validate_move, Move};
use crate::utils::EDGE_DISTANCES;
use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    /// Creates a new `PieceType`
    pub fn from(piece_type_str: String) -> Self {
        let piece_type_str = piece_type_str.chars().next().unwrap();
        let piece_type = match piece_type_str {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => panic!("Invalid piece type: {}", piece_type_str),
        };

        piece_type
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

/// Represents the contents of a single square: either empty or occupied by a Piece.
#[derive(Copy, Clone)]
enum Square {
    Empty,
    Piece(Piece),
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub bitboards: [u64; 12],
    //TODO add white and black occupancy bitboards
    pub all_white_bitboard: u64,
    pub all_black_bitboard: u64,
    /*
    0: White Pawns
    1: White Knights
    2: White Bishops
    3: White Rooks
    4: White Queens
    5: White King
    6: Black Pawns
    7: Black Knights
    8: Black Bishops
    9: Black Rooks
    10: Black Queens
    11: Black King
     */
    pub active_color: Color,
    pub castling_rights: u8,    // Use a bitmask for castling rights
    pub en_passant: Option<u8>, // Target square index for en passant
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    /// Parse the piece-placement field of a FEN (the first space-delimited part)
    /// into an array of 64 squares, where:
    ///
    /// - `squares[0]` is a1 (bottom-left),
    /// - `squares[7]` is h1,
    /// - `squares[8]` is a2, ...,
    /// - `squares[63]` is h8.
    ///
    /// The FEN ranks are given top-to-bottom: rank 8 first, then rank 7, etc.
    fn fen_to_positions(fen_board: &str) -> [Square; 64] {
        let mut squares = [Square::Empty; 64];

        let ranks: Vec<&str> = fen_board.split('/').collect();
        assert_eq!(
            ranks.len(),
            8,
            "FEN board must have 8 ranks separated by '/'"
        );

        // FEN rank 0 = top row (8th rank), rank 7 = bottom row (1st rank).
        // But in our squares array, rank 0 corresponds to squares[0..8] (bottom).
        // So we do: board_rank = 7 - fen_rank_index to invert top-to-bottom.
        for (fen_rank_index, rank_str) in ranks.iter().enumerate() {
            let board_rank = 7 - fen_rank_index;
            let mut file = 0;

            for ch in rank_str.chars() {
                match ch {
                    '1'..='8' => {
                        // A digit means N consecutive empty squares
                        let empty_count = ch.to_digit(10).unwrap() as usize;
                        file += empty_count;
                    }
                    'p' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Pawn,
                        });
                        file += 1;
                    }
                    'n' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Knight,
                        });
                        file += 1;
                    }
                    'b' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Bishop,
                        });
                        file += 1;
                    }
                    'r' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Rook,
                        });
                        file += 1;
                    }
                    'q' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Queen,
                        });
                        file += 1;
                    }
                    'k' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::Black,
                            piece_type: PieceType::King,
                        });
                        file += 1;
                    }
                    'P' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::Pawn,
                        });
                        file += 1;
                    }
                    'N' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::Knight,
                        });
                        file += 1;
                    }
                    'B' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::Bishop,
                        });
                        file += 1;
                    }
                    'R' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::Rook,
                        });
                        file += 1;
                    }
                    'Q' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::Queen,
                        });
                        file += 1;
                    }
                    'K' => {
                        squares[board_rank * 8 + file] = Square::Piece(Piece {
                            color: Color::White,
                            piece_type: PieceType::King,
                        });
                        file += 1;
                    }
                    // Ignore the slash itself — it’s part of the FEN rank separators
                    _ => panic!("Invalid FEN character: {}", ch),
                }
            }
            assert!(
                file <= 8,
                "FEN rank '{}' has too many squares (exceeds 8)",
                rank_str
            );
        }

        squares
    }

    /// Parse an entire FEN string into a `Board`.
    /// Expected format: "<piece-placements> <active_color> <castling> <en_passant> <halfmove> <fullmove>"
    pub fn fen_to_board(fen: &str) -> Board {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(parts.len(), 6, "Invalid FEN string");

        // 1) Piece placement
        let squares = Board::fen_to_positions(parts[0]);

        // 2) Active color
        let active_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid active color: {}", parts[1]),
        };

        // 3) Castling rights
        let mut castling_rights = 0;
        if parts[2].contains('K') {
            castling_rights |= 1; // White kingside
        }
        if parts[2].contains('Q') {
            castling_rights |= 1 << 1; // White queenside
        }
        if parts[2].contains('k') {
            castling_rights |= 1 << 2; // Black kingside
        }
        if parts[2].contains('q') {
            castling_rights |= 1 << 3; // Black queenside
        }

        // 4) En passant
        let en_passant = if parts[3] != "-" {
            let file = parts[3].chars().nth(0).unwrap() as u8 - 'a' as u8;
            let rank = parts[3].chars().nth(1).unwrap() as u8 - '1' as u8;
            Some(rank * 8 + file)
        } else {
            None
        };

        // 5) Halfmove clock
        let halfmove_clock = parts[4].parse::<u32>().unwrap();

        // 6) Fullmove number
        let fullmove_number = parts[5].parse::<u32>().unwrap();

        // Build bitboards based on squares
        let mut bitboards = [0u64; 12];
        let mut all_white_bitboard: u64 = 0;
        let mut all_black_bitboard: u64 = 0;
        for (sq_index, square) in squares.iter().enumerate() {
            if let Square::Piece(piece) = square {
                let piece_type_index = piece.piece_type as usize;
                let color_offset = piece.color as usize * 6;
                bitboards[color_offset + piece_type_index] |= 1 << sq_index;
                match piece.color {
                    Color::White => all_white_bitboard |= 1 << sq_index,
                    Color::Black => all_black_bitboard |= 1 << sq_index,
                }
            }
        }

        Board {
            bitboards,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
            all_white_bitboard: all_white_bitboard,
            all_black_bitboard: all_black_bitboard,
        }
    }

    /// Print a textual representation of the board to stdout.
    pub fn display(&self) {
        // Use these symbols to show the board (white first, then black)
        let piece_chars = [
            '♙', '♘', '♗', '♖', '♕', '♔', // White
            '♟', '♞', '♝', '♜', '♛', '♚', // Black
        ];

        // row 7 is top (rank 8) in typical text display, row 0 is bottom (rank 1)
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq_index = rank * 8 + file;
                let mut ch = '*';
                for (i, bitboard) in self.bitboards.iter().enumerate() {
                    if (bitboard & (1 << sq_index)) != 0 {
                        ch = piece_chars[i];
                        break;
                    }
                }
                print!("{} ", ch);
            }
            println!();
        }
        println!("Active Color: {:?}", self.active_color);
        println!(
            "Castling Rights: {}{}{}{}",
            if self.castling_rights & 1 != 0 {
                "K"
            } else {
                ""
            },
            if self.castling_rights & 2 != 0 {
                "Q"
            } else {
                ""
            },
            if self.castling_rights & 4 != 0 {
                "k"
            } else {
                ""
            },
            if self.castling_rights & 8 != 0 {
                "q"
            } else {
                ""
            },
        );
        if let Some(square) = self.en_passant {
            println!(
                "En Passant Target: {}{}",
                ('a' as u8 + (square % 8) as u8) as char,
                (square / 8) + 1
            );
        } else {
            println!("En Passant Target: None");
        }
        println!("Halfmove Clock: {}", self.halfmove_clock);
        println!("Fullmove Number: {}", self.fullmove_number);
    }

    /// Utility to print a 64-bit bitboard in an 8x8 grid to stdout.
    // pub fn display_bitboard(bitboard: u64) {
    //     for i in (0..64).rev() {
    //         if i % 8 == 7 {
    //             println!();
    //         }
    //         if (bitboard & (1 << i)) != 0 {
    //             print!("1");
    //         } else {
    //             print!("0");
    //         }
    //     }
    //     println!();
    // }

    /// Convert this Board back into a FEN string. If you store squares
    /// from a1..h1 up to a8..h8, you have to be careful to output ranks
    /// top-to-bottom.
    pub fn board_to_fen(&self) -> String {
        let mut fen = String::new();

        // For each rank from top (7) to bottom (0):
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let sq_index = rank * 8 + file;

                // Which (if any) piece index occupies this square?
                let piece_index_opt = self
                    .bitboards
                    .iter()
                    .position(|&bb| (bb & (1 << sq_index)) != 0);

                if let Some(piece_index) = piece_index_opt {
                    // If we had some empty squares prior, flush them into the FEN.
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    let piece_char = match piece_index {
                        0 => 'P',  // White Pawn
                        1 => 'N',  // White Knight
                        2 => 'B',  // White Bishop
                        3 => 'R',  // White Rook
                        4 => 'Q',  // White Queen
                        5 => 'K',  // White King
                        6 => 'p',  // Black Pawn
                        7 => 'n',  // Black Knight
                        8 => 'b',  // Black Bishop
                        9 => 'r',  // Black Rook
                        10 => 'q', // Black Queen
                        11 => 'k', // Black King
                        _ => panic!("Invalid piece index: {piece_index}"),
                    };
                    fen.push(piece_char);
                } else {
                    empty_count += 1;
                }
            }

            // If there are empty squares at the end of the rank, add them
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            // Separate ranks with '/'
            if rank > 0 {
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(match self.active_color {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling rights
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & 1 != 0 {
                fen.push('K');
            }
            if self.castling_rights & 2 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & 4 != 0 {
                fen.push('k');
            }
            if self.castling_rights & 8 != 0 {
                fen.push('q');
            }
        }

        // En passant
        fen.push(' ');
        if let Some(square) = self.en_passant {
            let file = (square % 8) as u8 + b'a';
            let rank = (square / 8) + 1;
            fen.push(file as char);
            fen.push_str(&rank.to_string());
        } else {
            fen.push('-');
        }

        // Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    pub fn move_peice(&mut self, m: Move) -> bool {
        let valid = validate_move(self, &m);

        let prev_board_state = self.clone();
        if valid {
            // Update the board state

            // 1. Move the piece

            // figure out what peice were moving
            let peice_type = find_peice_at_from_location(self, m.from);

            let peice_type = match peice_type {
                Some(peice_type) => peice_type,
                None => return false,
            };

            // en passant
            if peice_type == PieceType::Pawn
                && self.en_passant.is_some()
                && m.to == self.en_passant.unwrap()
            {
                // remove the pawn that is being taken
                let taken_peice_type = match self.active_color {
                    Color::White => PieceType::Pawn,
                    Color::Black => PieceType::Pawn,
                };

                let capture_bit = 1 << m.to - 8;
                let taken_peice_type = taken_peice_type as usize
                    + match self.active_color {
                        Color::White => 6,
                        Color::Black => 0,
                    };

                // remove the captured peice from the bitboard
                self.bitboards[taken_peice_type] &= !capture_bit;
            }

            // update the bitboards
            let enemy_bitboards = match self.active_color {
                Color::White => &self.bitboards[6..12],
                Color::Black => &self.bitboards[0..6],
            };

            // figure out if the piece is taking another piece
            let capture = match enemy_bitboards[peice_type as usize] & (1 << m.to) {
                0 => false,
                _ => true,
            };

            if capture {
                // find what kind of peice we are taking
                let to_bit = 1 << m.to;
                let taken_peice_type = enemy_bitboards.iter().position(|&bb| bb & to_bit != 0);

                // Once we find the type of peice we are taking, remove it from the enemy bitboard
                match taken_peice_type {
                    Some(taken_peice_type) => {
                        self.bitboards[taken_peice_type
                            + match self.active_color {
                                Color::White => 6,
                                Color::Black => 0,
                            }] &= !to_bit;
                    }
                    None => return false,
                }

                // remove from the all white or all black bitboard
                match self.active_color {
                    Color::White => self.all_black_bitboard &= !to_bit,
                    Color::Black => self.all_white_bitboard &= !to_bit,
                }
            }

            // Remove the peice being moved from the 'from' location for its given bitboard
            self.bitboards[peice_type as usize
                + match self.active_color {
                    Color::White => 0,
                    Color::Black => 6,
                }] &= !(1 << m.from);

            match self.active_color {
                Color::White => self.all_white_bitboard &= !(1 << m.from),
                Color::Black => self.all_black_bitboard &= !(1 << m.from),
            }

            // Placing our peice in its new location
            if m.promotion.is_none() {
                // Add the peice being moved to the 'to' location for its given bitboard
                self.bitboards[peice_type as usize
                    + match self.active_color {
                        Color::White => 0,
                        Color::Black => 6,
                    }] |= 1 << m.to;
            }
            // We are promoting the pawn and placing the new peice on the 'to' location
            else {
                self.bitboards[m.promotion.unwrap() as usize
                    + match self.active_color {
                        Color::White => 0,
                        Color::Black => 6,
                    }] |= 1 << m.to;
            }

            // no matter what we always update the all_white_bitboard or all_black_bitboard
            match self.active_color {
                Color::White => self.all_white_bitboard |= 1 << m.to,
                Color::Black => self.all_black_bitboard |= 1 << m.to,
            }

            // check to see if move puts the king in check or king is still in check
            if self.is_in_check(self.active_color) {
                *self = prev_board_state;
                return false;
            }
            self.display();
            // 2. Update castling rights
            if peice_type == PieceType::King {
                match m.from {
                    4 => {
                        // White King
                        self.castling_rights &= !(1 | 2);
                    }
                    60 => {
                        // Black King
                        self.castling_rights &= !(4 | 8);
                    }
                    _ => {}
                }
            } else if peice_type == PieceType::Rook {
                match m.from {
                    0 => {
                        // White Queenside Rook
                        self.castling_rights &= !1;
                    }
                    7 => {
                        // White Kingside Rook
                        self.castling_rights &= !2;
                    }
                    56 => {
                        // Black Queenside Rook
                        self.castling_rights &= !4;
                    }
                    63 => {
                        // Black Kingside Rook
                        self.castling_rights &= !8;
                    }
                    _ => {}
                }
            }
            // 3. Update en passant
            if peice_type == PieceType::Pawn && (m.from as i8 - m.to as i8).abs() == 16 {
                self.en_passant = Some((m.from + m.to) / 2); // set the en passant target square to the square behind the pawn
            } else {
                self.en_passant = None;
            }
            // 4. Update halfmove clock
            if peice_type == PieceType::Pawn || capture {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }
            // 5. Update fullmove number
            if self.active_color == Color::Black {
                self.fullmove_number += 1;
            }
            // 6. Switch active color
            self.active_color = match self.active_color {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }

        valid
    }

    // Given a color, return the bitboard squares being attacked by that color
    pub fn get_attack_bitboard_by_color(&self, color: Color) -> u64 {
        let mut attack_bitboard: u64 = 0;

        let offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };

        let pawns_bb = self.bitboards[offset + PieceType::Pawn as usize];

        let knights_bb = self.bitboards[offset + PieceType::Knight as usize];
        let bishops_bb = self.bitboards[offset + PieceType::Bishop as usize];
        let rooks_bb = self.bitboards[offset + PieceType::Rook as usize];
        let queens_bb = self.bitboards[offset + PieceType::Queen as usize];
        let king_bb = self.bitboards[offset + PieceType::King as usize];

        let board_occupancy_bb = self.all_white_bitboard | self.all_black_bitboard;
        attack_bitboard |= Self::get_pawn_attack_bitboard(pawns_bb, color);

        attack_bitboard |= Self::get_knight_attack_bitboard(knights_bb);

        attack_bitboard |= Self::get_bishop_attack_bitboard(bishops_bb, board_occupancy_bb);

        attack_bitboard |= Self::get_rook_attack_bitboard(rooks_bb, board_occupancy_bb);

        attack_bitboard |= Self::get_queen_attack_bitboard(queens_bb, board_occupancy_bb);

        attack_bitboard |= Self::get_king_attack_bitboard(king_bb);

        attack_bitboard
    }

    fn get_pawn_attack_bitboard(pawn_bb: u64, color: Color) -> u64 {
        let mut attack_bitboard: u64 = 0;

        match color {
            Color::White => {
                // Capture Right (East): Shift left by 9, exclude pawns on h-file
                attack_bitboard |= (pawn_bb & !0x8080808080808080) << 9;

                // Capture Left (West): Shift left by 7, exclude pawns on a-file
                attack_bitboard |= (pawn_bb & !0x0101010101010101) << 7;
            }
            Color::Black => {
                // capturing left => i -> i - 9, exclude h-file
                attack_bitboard |= (pawn_bb & !0x8080808080808080) >> 9;

                // capturing right => i -> i - 7, exclude a-file
                attack_bitboard |= (pawn_bb & !0x0101010101010101) >> 7;
            }
        }
        attack_bitboard
    }

    fn get_knight_attack_bitboard(knight_bb: u64) -> u64 {
        let mut attack_bitboard: u64 = 0;

        attack_bitboard |= (knight_bb << 17) & !0x8080808080808080; // Knight moves
        attack_bitboard |= (knight_bb << 15) & !0x0101010101010101;
        attack_bitboard |= (knight_bb << 10) & !0x8080808080808080;
        attack_bitboard |= (knight_bb << 6) & !0x0101010101010101;
        attack_bitboard |= (knight_bb >> 17) & !0x0101010101010101;
        attack_bitboard |= (knight_bb >> 15) & !0x8080808080808080;
        attack_bitboard |= (knight_bb >> 10) & !0x0101010101010101;
        attack_bitboard |= (knight_bb >> 6) & !0x8080808080808080;

        attack_bitboard
    }

    fn get_bishop_attack_bitboard(bishop_bb: u64, board_occpuancy_bb: u64) -> u64 {
        let mut attack_bitboard: u64 = 0;

        // Directions: NW (+7), NE (+9), SW (-9), SE (-7)
        let distance_to_jump: [i8; 4] = [9, 7, -7, -9]; // [NE, NW, SE, SW]
        let dir: [u8; 4] = [4, 5, 6, 7];

        for i in 0..4 {
            let mut temp_bb = bishop_bb;
            // Loop over every bishop to evaluate which squares they can attack
            while temp_bb != 0 {
                let square = temp_bb.trailing_zeros() as i8; // get the index of the first set bit aka that one of the bishops is on
                temp_bb &= temp_bb - 1; // remove the bit we just found

                let max_distance = EDGE_DISTANCES[dir[i] as usize][square as usize];

                for hop_distance_multiplier in 1..=max_distance {
                    let hop_distance = distance_to_jump[i] * hop_distance_multiplier as i8;

                    let attacking_square = square + hop_distance;

                    // Prevents wrapping around the board or going out of bounds
                    if attacking_square < 0 || attacking_square > 63 {
                        break;
                    }
                    let attacking_square_u8 = attacking_square as u8;
                    let attacking_bit = 1 << attacking_square_u8;

                    attack_bitboard |= attacking_bit;

                    // if we hit anny peice we stop
                    if board_occpuancy_bb & attacking_bit != 0 {
                        break;
                    }
                }
            }
        }
        attack_bitboard
    }

    fn get_rook_attack_bitboard(rook_bb: u64, board_occpuancy_bb: u64) -> u64 {
        let mut attack_bitboard: u64 = 0;

        // Define the distance offsets for Rook movement:
        // [North, South, East, West]
        let distance_to_jump: [i8; 4] = [8, -8, 1, -1];

        // Match these directions to EDGE_DISTANCES indices:
        // 0 = North, 1 = South, 2 = East, 3 = West
        let dir: [u8; 4] = [0, 1, 2, 3];

        for i in 0..4 {
            let mut temp_bb = rook_bb;
            // Loop over every bishop to evaluate which squares they can attack
            while temp_bb != 0 {
                let square = temp_bb.trailing_zeros() as i8; // get the index of the first set bit aka that one of the bishops is on
                temp_bb &= temp_bb - 1; // remove the bit we just found

                let max_distance = EDGE_DISTANCES[dir[i] as usize][square as usize];

                for hop_distance_multiplier in 1..=max_distance {
                    let hop_distance = distance_to_jump[i] * hop_distance_multiplier as i8;

                    let attacking_square = square + hop_distance;

                    // Prevents wrapping around the board or going out of bounds
                    if attacking_square < 0 || attacking_square > 63 {
                        break;
                    }
                    let attacking_square_u8 = attacking_square as u8;
                    let attacking_bit = 1 << attacking_square_u8;

                    attack_bitboard |= attacking_bit;

                    // if we hit anny peice we stop
                    if board_occpuancy_bb & attacking_bit != 0 {
                        break;
                    }
                }
            }
        }
        attack_bitboard
    }

    // just combine the boards of rook and bishop
    fn get_queen_attack_bitboard(queen_bb: u64, board_occpuancy_bb: u64) -> u64 {
        return Self::get_bishop_attack_bitboard(queen_bb, board_occpuancy_bb)
            | Self::get_rook_attack_bitboard(queen_bb, board_occpuancy_bb);
    }

    /// Given a bitboard of kings, returns a bitboard of squares they are attacking.
    pub fn get_king_attack_bitboard(king_bb: u64) -> u64 {
        let mut attack_bitboard: u64 = 0;

        // Define file masks to prevent wrapping
        const FILE_A: u64 = 0x0101010101010101;
        const FILE_H: u64 = 0x8080808080808080;

        // Kings can move in eight directions: N, NE, E, SE, S, SW, W, NW
        // Define each direction with its corresponding shift
        // North (N): Shift left by 8
        // South (S): Shift right by 8
        // East (E): Shift left by 1, but exclude h-file
        // West (W): Shift right by 1, but exclude a-file
        // Northeast (NE): Shift left by 9, but exclude h-file
        // Northwest (NW): Shift left by 7, but exclude a-file
        // Southeast (SE): Shift right by 7, but exclude h-file
        // Southwest (SW): Shift right by 9, but exclude a-file

        // North
        attack_bitboard |= king_bb << 8;

        // South
        attack_bitboard |= king_bb >> 8;

        // East
        attack_bitboard |= (king_bb & !FILE_H) << 1;

        // West
        attack_bitboard |= (king_bb & !FILE_A) >> 1;

        // Northeast
        attack_bitboard |= (king_bb & !FILE_H) << 9;

        // Northwest
        attack_bitboard |= (king_bb & !FILE_A) << 7;

        // Southeast
        attack_bitboard |= (king_bb & !FILE_H) >> 7;

        // Southwest
        attack_bitboard |= (king_bb & !FILE_A) >> 9;

        attack_bitboard
    }

    pub fn is_in_check(self, color: Color) -> bool {
        let king_bb = self.bitboards[color as usize * 6 + PieceType::King as usize];

        let attack_bb = self.get_attack_bitboard_by_color(match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        });

        (king_bb & attack_bb) != 0
    }

    pub fn is_insufficient_material(&self) -> bool {
        // Only kings left
        if self.all_white_bitboard.count_ones() == 1 && self.all_black_bitboard.count_ones() == 1 {
            return true;
        }

        // King and bishop/knight vs king
        if (self.all_white_bitboard.count_ones() == 2 && self.all_black_bitboard.count_ones() == 1) ||
           (self.all_white_bitboard.count_ones() == 1 && self.all_black_bitboard.count_ones() == 2) {
            let minor_pieces = self.bitboards[PieceType::Bishop as usize] | 
                             self.bitboards[PieceType::Knight as usize] |
                             self.bitboards[(PieceType::Bishop as usize) + 6] | 
                             self.bitboards[(PieceType::Knight as usize) + 6];
            if minor_pieces.count_ones() == 1 {
                return true;
            }
        }

        false
    }

    pub fn is_50_move_rule(&self) -> bool {
        self.halfmove_clock >= 50
    }

    pub fn is_3_fold_repetition(&self) -> bool {
        // TODO: Implement position history tracking for 3-fold repetition
        false
    }
}

// -------------------------------
// Tests
// -------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_to_positions_for_one_pawn() {
        // White pawn on e2
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let board = Board::fen_to_board(fen);
        // e2 => rank=1, file=4 => index = 1*8 + 4 = 12
        assert_eq!(board.bitboards[0], 1 << 12); // White Pawns are bitboards[0]
    }

    #[test]
    fn test_fen_to_position_for_two_pawns() {
        // White pawns on d2, e2 => (rank=1, file=3) and (rank=1, file=4)
        let fen = "8/8/8/8/8/8/3PP3/8 w - - 0 1";
        let board = Board::fen_to_board(fen);

        let expected_bitboard: u64 = (1 << 11) | (1 << 12);
        assert_eq!(board.bitboards[0], expected_bitboard);
    }

    #[test]
    fn test_fen_to_position_for_many_random_pawns() {
        let fen = "P6P/8/3pp3/8/8/8/3PP3/P6P w - - 0 1";
        let board = Board::fen_to_board(fen);

        let expected_white_bitboard: u64 =
            (1 << 11) | (1 << 12) | 1 << 0 | 1 << 7 | 1 << 56 | 1 << 63;
        let expected_black_bitboard: u64 = (1 << 44) | (1 << 43);
        assert_eq!(board.bitboards[0], expected_white_bitboard);
        assert_eq!(board.bitboards[6], expected_black_bitboard);
    }

    #[test]

    fn test_pawn_capture() {
        let fen = "8/8/8/8/8/5p2/4P3/8 w - - 0 1".to_string();

        let mut board = Board::fen_to_board(&fen);

        let m = Move {
            from: 12,
            to: 21,
            promotion: None,
        };

        assert!(board.move_peice(m));

        let expected_white_bitboard: u64 = 1 << 21;
        let expected_black_bitboard: u64 = 0;

        assert_eq!(board.bitboards[0], expected_white_bitboard);
        assert_eq!(board.bitboards[6], expected_black_bitboard);
    }

    #[test]
    fn test_pawn_promotion() {
        let fen = "8/4P3/8/8/8/8/8/8 w - - 0 1".to_string();

        let mut board = Board::fen_to_board(&fen);

        let m = Move::new("e7e8q".to_string());

        assert!(board.move_peice(m));

        let expected_white_bitboard: u64 = 1 << 60;
        assert_eq!(board.bitboards[4], expected_white_bitboard);
        assert_eq!(board.bitboards[0], 0);
    }

    #[test]
    fn test_pawn_promotion_and_capture() {
        let fen = "5p2/4P3/8/8/8/8/8/8 w - - 0 1".to_string();

        let mut board = Board::fen_to_board(&fen);

        let m = Move::new("e7f8q".to_string());

        assert!(board.move_peice(m));

        let expected_white_bitboard: u64 = 1 << 61;
        assert_eq!(board.bitboards[4], expected_white_bitboard);
        assert_eq!(board.bitboards[0], 0);
        assert_eq!(board.bitboards[6], 0);
    }

    #[test]
    fn test_all_bitboard_matches_combination_of_bitboards() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let board = Board::fen_to_board(&fen);

        let folded_white_bitboard = board.bitboards[0..6].iter().fold(0, |acc, &bb| acc | bb);
        let folded_black_bitboard = board.bitboards[6..12].iter().fold(0, |acc, &bb| acc | bb);

        assert_eq!(board.all_white_bitboard, folded_white_bitboard);
        assert_eq!(board.all_black_bitboard, folded_black_bitboard);
    }

    #[test]
    fn test_bitboards_matching_after_moves() {
        let fen = "rnbqkbnr/pppp1ppp/8/8/8/4p3/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut board = Board::fen_to_board(&fen);

        let m = Move::new("f2e3".to_string());

        assert!(board.move_peice(m));

        let folded_white_bitboard = board.bitboards[0..6].iter().fold(0, |acc, &bb| acc | bb);
        let folded_black_bitboard = board.bitboards[6..12].iter().fold(0, |acc, &bb| acc | bb);

        assert_eq!(board.all_white_bitboard, folded_white_bitboard);
        assert_eq!(board.all_black_bitboard, folded_black_bitboard);
    }

    #[test]
    fn test_white_bishops_attacking_bitboards() {
        let fen = "7B/8/8/8/8/8/8/8 b - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::White);

        let expected_attack_bitboard: u64 = 0x40201008040201;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_black_bishops_attacking_bitboards() {
        let fen = "7b/8/8/8/8/8/8/8 b - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::Black);

        let expected_attack_bitboard: u64 = 0x40201008040201;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_white_bishop_attacking_bitboards_when_spaces_occupied() {
        let fen = "7B/8/5p2/8/8/8/8/8 w - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::White);

        let expected_attack_bitboard: u64 = 1 << 54 | 1 << 45;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_pawn_attacking_bitboards() {
        let fen = "8/7P/8/8/8/8/8/8 w - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::White);

        let expected_attack_bitboard: u64 = 1 << 62;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_pawn_attacking_bitboards_two_squares() {
        let fen = "8/6P1/8/8/8/8/8/8 b - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::White);

        let expected_attack_bitboard: u64 = 1 << 61 | 1 << 63;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_pawn_attacking_bitboards_two_squares_overlapping() {
        let fen = "8/4P1P1/8/8/8/8/8/8 b - - 0 1";

        let board = Board::fen_to_board(&fen);

        let attack_bitboard = board.get_attack_bitboard_by_color(Color::White);

        let expected_attack_bitboard: u64 = 1 << 61 | 1 << 63 | 1 << 59;

        assert_eq!(attack_bitboard, expected_attack_bitboard);
    }

    #[test]
    fn test_if_black_king_in_check() {
        let fen = "7k/8/8/8/8/2B5/8/8 w - - 0 1";

        let board = Board::fen_to_board(&fen);

        assert!(board.is_in_check(Color::Black));
    }

    #[test]
    fn test_if_black_king_is_not_in_check_due_to_wrong_diagonal() {
        let fen = "6k1/8/8/8/8/2B5/8/8 w - - 0 1";

        let board = Board::fen_to_board(&fen);

        // Assert we are not in check
        assert!(!board.is_in_check(Color::Black));
    }

    #[test]
    fn test_if_white_queen_in_check() {
        let fen = "8/8/8/8/8/5pp1/5pp1/7K w - - 0 1";

        let board = Board::fen_to_board(&fen);

        assert!(board.is_in_check(Color::White));
    }
}
