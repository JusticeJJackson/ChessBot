use board::{Board, Color};
use chess_move::{is_in_checkmate, validate_move, Move};
use std::io::{self, Write};

mod board;
mod chess_move;
mod utils;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/*
A  B  C  D  E  F  G  H

56 57 58 59 60 61 62 63   8
48 49 50 51 52 53 54 55   7
40 41 42 43 44 45 46 47   6
32 33 34 35 36 37 38 39   5
24 25 26 27 28 29 30 31   4
16 17 18 19 20 21 22 23   3
 8  9 10 11 12 13 14 15   2
 0  1  2  3  4  5  6  7   1
 */
fn main() {
    let mut game_board = Board::fen_to_board(STARTING_FEN);

    game_board.display();
    loop {
        print!("Enter your move in UCI format: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let m = Move::new(input.to_string());
        let chess_move = validate_move(&game_board, &m);

        if !chess_move {
            println!("Invalid move");
            continue;
        }

        game_board.move_peice(m);

        if game_board.is_in_check(match game_board.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }) {
            println!("Check!");

            if is_in_checkmate(&game_board) {
                println!("Checkmate!");
                break;
            }
        }

        if utils::is_stalemate(&game_board, match game_board.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }) {
            println!("Game Over - Stalemate!");
            break;
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        game_board.display();
    }
}

//TODO
/*
Stalemate detection
 */
