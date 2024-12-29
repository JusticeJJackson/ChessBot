pub struct Board {
    pub bitboards: [u64; 12],
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
    pub castling_rights: u8,       // Use a bitmask for castling rights
    pub en_passant: Option<usize>, // Target square index for en passant
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
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

#[derive(Copy, Clone)]
enum Square {
    Empty,
    Piece(Piece),
}
impl Board {
    fn fen_to_positions(fen: &str) -> [Square; 64] {
        let mut squares = [Square::Empty; 64];
        let mut i = 0;

        for c in fen.chars() {
            match c {
                'p' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Pawn,
                    });
                    i += 1;
                }
                'n' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Knight,
                    });
                    i += 1;
                }
                'b' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Bishop,
                    });
                    i += 1;
                }
                'r' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Rook,
                    });
                    i += 1;
                }
                'q' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Queen,
                    });
                    i += 1;
                }
                'k' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::Black,
                        piece_type: PieceType::King,
                    });
                    i += 1;
                }
                'P' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::Pawn,
                    });
                    i += 1;
                }
                'N' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::Knight,
                    });
                    i += 1;
                }
                'B' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::Bishop,
                    });
                    i += 1;
                }
                'R' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::Rook,
                    });
                    i += 1;
                }
                'Q' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::Queen,
                    });
                    i += 1;
                }
                'K' => {
                    squares[i] = Square::Piece(Piece {
                        color: Color::White,
                        piece_type: PieceType::King,
                    });
                    i += 1;
                }
                '/' => (), // Skip row separator
                '1'..='8' => {
                    let n = c.to_digit(10).unwrap() as usize;
                    i += n; // Advance the index by the number of empty squares
                }
                _ => panic!("Invalid FEN character: {}", c),
            }
        }

        squares
    }

    pub fn fen_to_board(fen: &str) -> Board {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(parts.len(), 6, "Invalid FEN string");

        let squares = Board::fen_to_positions(parts[0]);

        // Parse active color
        let active_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid active color: {}", parts[1]),
        };

        // Parse castling rights
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

        // Parse en passant square
        let en_passant = if parts[3] != "-" {
            let file = parts[3].chars().nth(0).unwrap() as usize - 'a' as usize;
            let rank = parts[3].chars().nth(1).unwrap() as usize - '1' as usize;
            Some(rank * 8 + file)
        } else {
            None
        };

        // Parse halfmove clock
        let halfmove_clock = parts[4].parse::<u32>().unwrap();

        // Parse fullmove number
        let fullmove_number = parts[5].parse::<u32>().unwrap();

        // Build bitboards
        let mut bitboards = [0; 12];
        // reverse the squares to get the correct order
        for (i, square) in squares.iter().rev().enumerate() {
            match square {
                Square::Piece(piece) => {
                    let piece_type = piece.piece_type as usize;
                    let color = piece.color as usize;
                    bitboards[(color * 6) + piece_type] |= 1 << i;
                }
                Square::Empty => (),
            }
        }

        Board {
            bitboards,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }
    pub fn display(&self) {
        let mut board = [['*'; 8]; 8]; // Initialize the board with '*' for empty spots
        let piece_chars = [
            '♙', '♘', '♗', '♖', '♕', '♔', // White pieces
            '♟', '♞', '♝', '♜', '♛', '♚', // Black pieces
        ];
        for (i, bitboard) in self.bitboards.iter().enumerate() {
            for j in 0..64 {
                if bitboard & (1 << j) != 0 {
                    board[j / 8][j % 8] = piece_chars[i];
                }
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", board[i][j]);
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

    pub fn display_bitboard(bitboard: u64) {
        for i in (0..64).rev() {
            if i % 8 == 0 {
                println!();
            }
            print!("{}", if bitboard & (1 << i) != 0 { "1" } else { "0" });
        }
        println!();
    }

    pub fn board_to_fen(&self) -> String {
        let mut fen = String::new();

        // Convert bitboards to a FEN position string
        for rank in 0..8 {
            let mut empty_count = 0;

            for file in 0..8 {
                let square_index = rank * 8 + file;

                // Check which piece occupies the square, if any
                let piece = self
                    .bitboards
                    .iter()
                    .position(|&bitboard| bitboard & (1 << square_index) != 0);

                if let Some(piece) = piece {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    let piece_char = match piece {
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
                        _ => panic!("Invalid piece index"),
                    };

                    fen.push(piece_char);
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

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

        // En passant target square
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_to_positions_for_one_pawn() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let squares = Board::fen_to_board(fen);

        assert_eq!(squares.bitboards[0], 1 << 12);
    }

    #[test]
    fn test_fen_to_position_for_two_pawns() {
        let fen = "8/8/8/8/8/8/3PP3/8 w - - 0 1";
        let squares = Board::fen_to_board(fen);

        let test_bitboard: u64 = 1 << 12 | 1 << 11;
        assert_eq!(squares.bitboards[0], test_bitboard);
    }
}
