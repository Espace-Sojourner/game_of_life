
extern crate board_crate;

#[cfg(test)]
mod tests {
    use super::*;
    use board_crate::Board;

    #[test]
    fn get_cell_returns_some() 
    {
        let board = Board::new(5, 5);

        assert_eq!(board.get_cell(0, 0), Some(0));
    }

    #[test]
    fn get_cell_returns_none()
    {
        let board = Board::new(5, 5);

        assert_eq!(board.get_cell(10, 10), None);
    }
}