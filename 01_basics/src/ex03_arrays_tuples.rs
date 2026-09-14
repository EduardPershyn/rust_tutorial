//! Exercise 03: arrays, tuples, Option, nested loops.
//! Run: cargo test ex03
//! Use plain loops, not iterator helpers like `.iter().sum()` / `.min()` / `.position()`.

/// Sum of all elements.
pub fn sum(arr: [i32; 5]) -> i32 {
    todo!()
}

/// (min, max) in a single pass.
pub fn min_max(arr: [i32; 5]) -> (i32, i32) {
    todo!()
}

/// Index of the first occurrence of `target`, or None.
pub fn find_index(arr: [i32; 5], target: i32) -> Option<usize> {
    todo!()
}

/// A new array with elements in reverse order.
/// Hint: parameters are immutable, but you can write `mut arr: [i32; 5]`.
pub fn reversed(arr: [i32; 5]) -> [i32; 5] {
    todo!()
}

/// First pair of indices (i, j) with i < j and arr[i] + arr[j] == target.
/// "First" = smallest i, then smallest j. None if there is no such pair.
pub fn two_sum(arr: [i32; 5], target: i32) -> Option<(usize, usize)> {
    todo!()
}

/// Tic-tac-toe: board cells are 'X', 'O' or '.' (empty).
/// Return Some('X') / Some('O') if that player has 3 in a row
/// (row, column or diagonal), otherwise None. At most one player wins.
pub fn winner(board: [[char; 3]; 3]) -> Option<char> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_values() {
        assert_eq!(sum([1, 2, 3, 4, 5]), 15);
        assert_eq!(sum([-1, 1, -1, 1, 0]), 0);
    }

    #[test]
    fn min_max_values() {
        assert_eq!(min_max([3, 1, 4, 1, 5]), (1, 5));
        assert_eq!(min_max([-7, -2, -9, -1, -5]), (-9, -1));
        assert_eq!(min_max([4, 4, 4, 4, 4]), (4, 4));
    }

    #[test]
    fn find_index_values() {
        assert_eq!(find_index([10, 20, 30, 20, 10], 20), Some(1));
        assert_eq!(find_index([10, 20, 30, 20, 10], 10), Some(0));
        assert_eq!(find_index([10, 20, 30, 20, 10], 99), None);
    }

    #[test]
    fn reversed_values() {
        assert_eq!(reversed([1, 2, 3, 4, 5]), [5, 4, 3, 2, 1]);
        assert_eq!(reversed([0, 0, 1, 0, 0]), [0, 0, 1, 0, 0]);
    }

    #[test]
    fn two_sum_values() {
        assert_eq!(two_sum([1, 2, 3, 4, 5], 5), Some((0, 3)));
        assert_eq!(two_sum([1, 2, 3, 4, 5], 9), Some((3, 4)));
        assert_eq!(two_sum([5, 5, 5, 5, 5], 10), Some((0, 1)));
        assert_eq!(two_sum([1, 2, 3, 4, 5], 2), None); // can't use the same element twice
        assert_eq!(two_sum([1, 2, 3, 4, 5], 100), None);
    }

    #[test]
    fn winner_rows_and_columns() {
        let row = [['O', 'O', '.'], ['X', 'X', 'X'], ['.', '.', '.']];
        assert_eq!(winner(row), Some('X'));
        let col = [['X', 'O', 'X'], ['.', 'O', 'X'], ['X', 'O', '.']];
        assert_eq!(winner(col), Some('O'));
    }

    #[test]
    fn winner_diagonals() {
        let diag = [['O', 'X', '.'], ['X', 'O', '.'], ['.', 'X', 'O']];
        assert_eq!(winner(diag), Some('O'));
        let anti = [['O', 'O', 'X'], ['.', 'X', '.'], ['X', '.', '.']];
        assert_eq!(winner(anti), Some('X'));
    }

    #[test]
    fn winner_none() {
        let empty = [['.'; 3]; 3];
        assert_eq!(winner(empty), None); // three '.' in a row is not a win
        let draw = [['X', 'O', 'X'], ['X', 'O', 'O'], ['O', 'X', 'X']];
        assert_eq!(winner(draw), None);
    }
}
