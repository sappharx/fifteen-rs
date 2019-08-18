use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub struct Board {
    size: u8,
    grid: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(size: u8) -> Board {
        let mut grid = Vec::with_capacity(size as usize);
        for i in 0..size {
            let mut row = Vec::with_capacity(size as usize);

            for j in 0..size {
                row.push( get_starting_index(size, i, j) );
            }

            grid.push( row );
        }

        Board { size, grid }
    }

    pub fn move_tile(&mut self, tile: u8) -> Result<(), Error> {
        if tile == 0 {
            return Err( Error::new("can't move empty tile".to_string()) );
        }

        let zero_coords = self.get_coordinates(0);
        let tile_coords = self.get_coordinates(tile);

        if let None = zero_coords {
            panic!("can't find empty space on board");
        }

        if let None = tile_coords {
            return Err( Error::new("tile doesn't exist on board".to_string()) );
        }

        match (zero_coords, tile_coords) {
            (None, _) => { panic!("can't find empty space on board"); },
            (_, None) => { return Err( Error::new("tile doesn't exist on board".to_string()) ); },
            (Some(zero_coords), Some(tile_coords)) => {
                let is_valid_move = {
                    let same_row = tile_coords.row == zero_coords.row;
                    let same_col = tile_coords.col == zero_coords.col;
                    let adjacent_row = i32::abs( tile_coords.row as i32 - zero_coords.row as i32 ) == 1;
                    let adjacent_col = i32::abs( tile_coords.col as i32 - zero_coords.col as i32 ) == 1;

                    match (same_row, same_col, adjacent_row, adjacent_col) {
                        (true, false, false, true) => true,
                        (false, true, true, false) => true,
                        (_, _, _, _) => false,
                    }
                };

                if is_valid_move {
                    self.swap(tile_coords, zero_coords);
                    Ok(())
                } else {
                    Err(Error::new("invalid move".to_string()))
                }
            }
        }
    }

    fn get_coordinates(&self, tile: u8) -> Option<Coordinates> {
        for i in 0..self.size as usize {
            for j in 0..self.size as usize {
                if self.grid[i][j] == tile {
                    return Some( Coordinates { row: i, col: j });
                }
            }
        }

        None
    }

    fn swap(&mut self, a: Coordinates, b: Coordinates) {
        let tmp = self.grid[a.row][a.col];

        self.grid[a.row][a.col] = self.grid[b.row][b.col];
        self.grid[b.row][b.col] = tmp;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinates {
    row: usize,
    col: usize,
}

fn get_starting_index(size: u8, row: u8, col: u8) -> u8 {
    size * size - row * size - (col + 1)
}

#[cfg(test)]
mod tests {
    use crate::{Board,Coordinates, get_starting_index };

    #[test]
    fn test_board_get_coordinates() {
        let board = Board::new(4);
        assert_eq!(board.get_coordinates(15), Some(Coordinates{ row: 0, col: 0 }));
        assert_eq!(board.get_coordinates(9), Some(Coordinates{ row: 1, col: 2 }));
    }

    #[test]
    fn test_board_swap() {
        let mut board = Board::new(4);

        let a = Coordinates { row: 3, col: 2};
        let b = Coordinates { row: 3, col: 3};
        board.swap(a, b);

        assert_eq!(board.grid[3][2], 0);
        assert_eq!(board.grid[3][3], 1);
    }

    #[test]
    fn test_get_starting_index() {
        assert_eq!(get_starting_index(4, 0, 0), 15);
        assert_eq!(get_starting_index(4, 3, 3), 0);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }

    pub fn to_string(self) -> String {
        self.into()
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.message
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.message.fmt(formatter)
    }
}
