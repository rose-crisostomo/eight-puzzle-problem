#[cfg(test)]
mod tests {
    use crate::{expand, move_cursor, swap, Actions, Board};
    // use super::*;

    #[test]
    fn test_swap() {
        let mut board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        swap(&mut board, (0, 0), (0, 1)); // swap 7 and 2
        assert_eq!(board, [[2, 7, 4], [5, 0, 6], [8, 3, 1]]);
    }

    #[test]
    fn test_move_left_valid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 1), Actions::LEFT, 0, 0);
        if let Some(result) = result {
            assert_eq!(result.cursor, (1, 0));
            assert_eq!(result.board, [[7, 2, 4], [0, 5, 6], [8, 3, 1]]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_left_invalid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 0), Actions::LEFT, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_move_right_valid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 1), Actions::RIGHT, 0, 0);
        if let Some(result) = result {
            assert_eq!(result.cursor, (1, 2));
            assert_eq!(result.board, [[7, 2, 4], [5, 6, 0], [8, 3, 1]]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_right_invalid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 2), Actions::RIGHT, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_move_up_valid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 1), Actions::UP, 0, 0);
        if let Some(result) = result {
            assert_eq!(result.cursor, (0, 1));
            assert_eq!(result.board, [[7, 0, 4], [5, 2, 6], [8, 3, 1]]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_up_invalid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (0, 1), Actions::UP, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_move_down_valid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (1, 1), Actions::DOWN, 0, 0);
        if let Some(result) = result {
            assert_eq!(result.cursor, (2, 1));
            assert_eq!(result.board, [[7, 2, 4], [5, 3, 6], [8, 0, 1]]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_down_invalid() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = move_cursor(board, (3, 1), Actions::DOWN, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_expand() {
        let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
        let result = expand(board, (1, 1), 0, 0);
        assert_eq!(result.len(), 4)
    }
}