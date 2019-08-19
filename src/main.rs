use fifteen::Board;

fn main() {
    let mut board = Board::new(4);

    println!("{:?}", board);
    println!("{}", board);

    let moves = [1, 5, 13];

    for x in moves.iter() {
        match board.move_tile(*x) {
            Ok(()) => {
                println!("{:?}", board);
                println!("{}", board)
            }
            Err(e) => eprintln!("{}", e),
        };
    }
}
