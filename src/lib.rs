mod coordinates;
pub mod error;
mod util;

use std::fmt;

use coordinates::Coordinates;
use error::Error;
use util::get_starting_index;

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
                row.push(get_starting_index(size, i, j));
            }

            grid.push(row);
        }

        Board { size, grid }
    }

    pub fn move_tile(&mut self, tile: u8) -> Result<(), Error> {
        if tile == 0 {
            return Err(Error::new("can't move empty tile".to_string()));
        }

        let zero_coords = self.get_coordinates(0);
        let tile_coords = self.get_coordinates(tile);

        if let None = zero_coords {
            panic!("can't find empty space on board");
        }

        if let None = tile_coords {
            return Err(Error::new("tile doesn't exist on board".to_string()));
        }

        match (zero_coords, tile_coords) {
            (None, _) => {
                panic!("can't find empty space on board");
            }
            (_, None) => {
                return Err(Error::new("tile doesn't exist on board".to_string()));
            }
            (Some(zero_coords), Some(tile_coords)) => {
                let is_valid_move = {
                    let same_row = tile_coords.is_same_row(zero_coords);
                    let same_col = tile_coords.is_same_col(zero_coords);
                    let adjacent_row = tile_coords.is_adjacent_row(zero_coords);
                    let adjacent_col = tile_coords.is_adjacent_col(zero_coords);

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
                    return Some(Coordinates::new(i, j));
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let max_len = u8::to_string(&self.size.pow(2)).len() as usize;

        for row in 0..self.size {
            for col in 0..self.size {
                let val = self.grid[row as usize][col as usize];
                let output = match val {
                    0 => "".to_string(),
                    _ => val.to_string(),
                };
                write!(f, " {:>width$}", output, width = max_len)?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Coordinates};

    #[test]
    fn test_get_coordinates() {
        let board = Board::new(4);
        assert_eq!(
            board.get_coordinates(15),
            Some(Coordinates { row: 0, col: 0 })
        );
        assert_eq!(
            board.get_coordinates(9),
            Some(Coordinates { row: 1, col: 2 })
        );
    }

    #[test]
    fn test_swap() {
        let mut board = Board::new(4);

        let a = Coordinates { row: 3, col: 2 };
        let b = Coordinates { row: 3, col: 3 };
        board.swap(a, b);

        assert_eq!(board.grid[3][2], 0);
        assert_eq!(board.grid[3][3], 1);
    }

}
