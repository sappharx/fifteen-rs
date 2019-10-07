use std::io::{self, Write};

use fifteen::Board;

fn main() {
    let mut board = Board::random(4);
    // let mut board = Board::new(4);

    loop {
        // clear screen
        print!("{}[2J", 27 as char);

        // draw board
        println!("{}", board);

        // check if won
        if board.is_complete() {
            println!("FTW!!!");
            break;
        }

        print!("your move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(x) => {
                        match board.move_tile(x) {
                            Ok(()) => {}
                            Err(e) => println!("{}", e),
                        };
                    }
                    Err(e) => println!("invalid input: {}", e),
                };
            }
            Err(e) => println!("error reading from stdin: {}", e),
        };
    }
}
