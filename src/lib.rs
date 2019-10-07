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

    fn from_list(size: u8, list: Vec<u8>) -> Result<Board, Error> {
        if ( list.len() as u8 ) < size * size {
            return Err(Error::new(format!("not enough elements in list; expected {}, got {}", size * size, list.len())));
        }

        let mut grid = Vec::with_capacity(size as usize);

        for i in 0..size {
            let mut row = Vec::with_capacity(size as usize);

            for j in 0..size {
                let idx = size * i as u8 + j as u8;
                row.push(list[idx as usize]);
            }

            grid.push(row);
        }

        Ok(Board { size, grid } )
    }

    pub fn is_complete(&self) -> bool {
        for i in 0..self.size as usize {
            for j in 0..self.size as usize {
                let max: usize = self.size as usize - 1;
                let expected = if i == max && j == max {
                    0
                } else {
                    1 + self.size * i as u8 + j as u8
                };

                if self.grid[i][j] != expected {
                    println!("expected: {}; got: {}", expected, self.grid[i][j]);
                    return false;
                }
            }
        }

        true
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
    fn test_from_list() {
        let board = Board::from_list(4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,14, 15, 0]).unwrap();

        assert_eq!(board.grid[0][0], 1);
        assert_eq!(board.grid[0][1], 2);
        assert_eq!(board.grid[0][2], 3);
        assert_eq!(board.grid[0][3], 4);

        assert_eq!(board.grid[1][0], 5);
        assert_eq!(board.grid[1][1], 6);
        assert_eq!(board.grid[1][2], 7);
        assert_eq!(board.grid[1][3], 8);

        assert_eq!(board.grid[2][0], 9);
        assert_eq!(board.grid[2][1], 10);
        assert_eq!(board.grid[2][2], 11);
        assert_eq!(board.grid[2][3], 12);

        assert_eq!(board.grid[3][0], 13);
        assert_eq!(board.grid[3][1], 14);
        assert_eq!(board.grid[3][2], 15);
        assert_eq!(board.grid[3][3], 0);
    }

    #[test]
    fn test_is_complete() {
        let complete_board = Board::from_list(4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,14, 15, 0]).unwrap();
        let incomplete_board = Board::from_list(4, vec![2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,14, 15, 0]).unwrap();

        assert!(complete_board.is_complete());
        assert!(!incomplete_board.is_complete());
    }

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
