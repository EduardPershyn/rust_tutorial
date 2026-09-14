//! Exercise 01: borrowing numbers through slices: &[T], &mut [T], &mut Vec<T>.
//! Run: cargo test --lib ex01
//! Loop over the elements (`for x in nums`), not over indexes, unless you really need an index.

/// Sum of all elements. The same function works for arrays, Vecs and sub-slices.
pub fn sum(nums: &[i64]) -> i64 {
    todo!()
}

/// The largest element, or None for an empty slice.
pub fn largest(nums: &[i32]) -> Option<i32> {
    todo!()
}

/// Multiply every element by 2, in place.
pub fn double_in_place(nums: &mut [i32]) {
    todo!()
}

/// Replace each element with the sum of itself and all elements before it, in place.
/// [1, 2, 3, 4] → [1, 3, 6, 10]
pub fn running_sum(nums: &mut [i64]) {
    todo!()
}

/// Split into two halves. For an odd length, the extra element goes to the second half.
/// [1, 2, 3, 4, 5] → ([1, 2], [3, 4, 5]). Both results borrow from `nums`; nothing is copied.
pub fn halves(nums: &[i32]) -> (&[i32], &[i32]) {
    todo!()
}

/// Remove all zeros, keeping the order of the other elements.
/// This takes `&mut Vec`, not `&mut [i32]`, because a slice can't change its length.
/// Don't use `retain` (it needs a closure, Section 08).
/// Hint: build a new Vec, then replace the caller's Vec with `*nums = new_vec;`.
pub fn remove_zeros(nums: &mut Vec<i32>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_any_source() {
        let arr = [1, 2, 3, 4];
        let v = vec![10, 20, 30];
        assert_eq!(sum(&arr), 10);
        assert_eq!(sum(&v), 60);
        assert_eq!(sum(&v[1..]), 50);
        assert_eq!(sum(&[]), 0);
        assert_eq!(v.len(), 3); // still ours: sum only borrowed it
    }

    #[test]
    fn largest_values() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[-5, -1, -7]), Some(-1));
        assert_eq!(largest(&[42]), Some(42));
        assert_eq!(largest(&[]), None);
    }

    #[test]
    fn double_whole_and_part() {
        let mut v = vec![1, -2, 3];
        double_in_place(&mut v);
        assert_eq!(v, [2, -4, 6]);

        let mut arr = [1, 1, 1, 1];
        double_in_place(&mut arr[1..3]); // only the middle
        assert_eq!(arr, [1, 2, 2, 1]);
    }

    #[test]
    fn running_sum_values() {
        let mut v = vec![1, 2, 3, 4];
        running_sum(&mut v);
        assert_eq!(v, [1, 3, 6, 10]);

        let mut w = vec![5, -5, 5];
        running_sum(&mut w);
        assert_eq!(w, [5, 0, 5]);

        running_sum(&mut []); // must not panic
    }

    #[test]
    fn halves_values() {
        let v = vec![1, 2, 3, 4, 5];
        let (left, right) = halves(&v);
        assert_eq!(left, [1, 2]);
        assert_eq!(right, [3, 4, 5]);

        let (left, right) = halves(&[7, 8]);
        assert_eq!(left, [7]);
        assert_eq!(right, [8]);

        let (left, right) = halves(&[]);
        assert!(left.is_empty() && right.is_empty());
    }

    #[test]
    fn remove_zeros_values() {
        let mut v = vec![0, 1, 0, 0, 2, 3, 0];
        remove_zeros(&mut v);
        assert_eq!(v, [1, 2, 3]);

        let mut all_zero = vec![0, 0];
        remove_zeros(&mut all_zero);
        assert!(all_zero.is_empty());
    }
}
