pub fn get_starting_index(size: u8, row: u8, col: u8) -> u8 {
    size * size - row * size - (col + 1)
}

#[cfg(test)]
mod tests {
    use crate::get_starting_index;

    #[test]
    fn test_get_starting_index() {
        assert_eq!(get_starting_index(4, 0, 0), 15);
        assert_eq!(get_starting_index(4, 3, 3), 0);
    }
}
