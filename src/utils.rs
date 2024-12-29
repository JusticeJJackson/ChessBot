use once_cell::sync::Lazy;

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

// Directions: [North, South, East, West, North-East, North-West, South-East, South-West]
pub static EDGE_DISTANCES: Lazy<[Vec<u8>; 8]> = Lazy::new(|| {
    let mut north: Vec<u8> = vec![0; 64];
    let mut south: Vec<u8> = vec![0; 64];
    let mut east: Vec<u8> = vec![0; 64];
    let mut west: Vec<u8> = vec![0; 64];
    let mut north_east: Vec<u8> = vec![0; 64];
    let mut north_west: Vec<u8> = vec![0; 64];
    let mut south_east: Vec<u8> = vec![0; 64];
    let mut south_west: Vec<u8> = vec![0; 64];

    for square in 0..64 {
        let row = square / 8;
        let col = square % 8;

        north[square] = (7 - row) as u8;
        south[square] = (row) as u8;
        east[square] = (7 - col) as u8;
        west[square] = (col) as u8;
        north_east[square] = (north[square].min(east[square])) as u8;
        north_west[square] = (north[square].min(west[square])) as u8;
        south_east[square] = (south[square].min(east[square])) as u8;
        south_west[square] = (south[square].min(west[square])) as u8;
    }

    [
        north, south, east, west, north_east, north_west, south_east, south_west,
    ]
});
