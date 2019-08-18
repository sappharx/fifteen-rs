#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl Coordinates {
    pub fn new(row: usize, col: usize) -> Coordinates {
        Coordinates { row, col }
    }

    pub fn is_same_row(&self, other: Self) -> bool {
        self.row == other.row
    }

    pub fn is_same_col(&self, other: Self) -> bool {
        self.col == other.col
    }

    pub fn is_adjacent_row(&self, other: Self) -> bool {
        i32::abs(self.row as i32 - other.row as i32) == 1
    }

    pub fn is_adjacent_col(&self, other: Self) -> bool {
        i32::abs(self.col as i32 - other.col as i32) == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Coordinates;

    #[test]
    fn test_is_same_row() {
        let a = Coordinates::new(0, 0);

        assert!(a.is_same_row(Coordinates::new(0, 3)));
        assert!(!a.is_same_row(Coordinates::new(1, 3)));
        assert!(!a.is_same_row(Coordinates::new(2, 3)));
    }

    #[test]
    fn test_is_same_col() {
        let a = Coordinates::new(0, 0);

        assert!(a.is_same_col(Coordinates::new(1, 0)));
        assert!(!a.is_same_col(Coordinates::new(0, 1)));
        assert!(!a.is_same_col(Coordinates::new(1, 3)));
    }

    #[test]
    fn test_is_adjacent_row() {
        let a = Coordinates::new(0, 0);

        assert!(!a.is_adjacent_row(Coordinates::new(0, 3)));
        assert!(a.is_adjacent_row(Coordinates::new(1, 3)));
        assert!(!a.is_adjacent_row(Coordinates::new(2, 3)));
    }

    #[test]
    fn test_is_adjacent_col() {
        let a = Coordinates::new(0, 0);

        assert!(!a.is_adjacent_col(Coordinates::new(0, 3)));
        assert!(a.is_adjacent_col(Coordinates::new(1, 1)));
        assert!(!a.is_adjacent_col(Coordinates::new(2, 0)));
    }
}
