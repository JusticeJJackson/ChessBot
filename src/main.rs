use board::Board;

mod board;
mod chess_move;

fn main() {
    let game_board =
        Board::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    game_board.display();

    let fen = game_board.board_to_fen();

    println!("{}", fen);
}
