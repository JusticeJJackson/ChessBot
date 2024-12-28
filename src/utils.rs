pub fn convert_board_coordinate_to_idx(board_coordinate: String) -> u8 {
    let mut board_coordinate = board_coordinate.chars();
    let file = board_coordinate.next().unwrap();
    let rank = board_coordinate.next().unwrap();

    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid file"),
    };

    let rank = match rank {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("Invalid rank"),
    };

    (rank * 8 + file) as u8 // Return the index of the square
}
