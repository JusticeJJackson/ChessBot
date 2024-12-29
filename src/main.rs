use board::Board;
use chess_move::validate_move;

mod board;
mod chess_move;
mod utils;

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
    let game_board =
        Board::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // game_board.display();

    let m = chess_move::Move::new("e2e4".to_string());

    let valid = validate_move(&game_board, &m);
    println!("{}", valid);
}
