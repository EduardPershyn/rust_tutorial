//! Exercise 01: Vec in practice: sorting, dedup, windows, chunks, join.
//! Run: cargo test --lib ex01
//! Allowed: simple closures for `sort_by_key` / `retain`, and `.collect()`.

/// Sorted values with duplicates removed. `values` must not be modified.
/// Hint: `to_vec()` copies a slice into a new Vec; then `sort` and `dedup`.
pub fn sorted_unique(values: &[i32]) -> Vec<i32> {
    let mut vector = values.to_vec();
    vector.sort();
    vector.dedup();
    vector
}

/// The median: the middle value of the sorted data, or the average of the two middle
/// values when the length is even. None for an empty slice. Don't modify `values`.
pub fn median(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    let mid = sorted.len() / 2;

    if !sorted.len().is_multiple_of(2) {
        Some(sorted[mid] as f64)
    } else {
        Some((sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0)
    }
}

/// Sum of each consecutive group of `size` elements; the last group may be smaller.
/// chunk_sums(&[1, 2, 3, 4, 5], 2) == [3, 7, 5]. Empty result for size 0 or no values.
pub fn chunk_sums(values: &[i32], size: usize) -> Vec<i32> {
    if size == 0 {
        return Vec::new();               // chunks(0) would PANIC, so guard first
    }
    let mut sums = Vec::new();
    for chunk in values.chunks(size) {   // chunk: &[i32], e.g. [1, 2], then [3, 4], then [5]
        let mut sum = 0;
        for x in chunk {                 // the inner loop just adds up one group
            sum += x;
        }
        sums.push(sum);
    }
    sums
}

/// The largest sum of any `k` CONSECUTIVE elements.
/// None if k is 0 or larger than the slice. max_window_sum(&[1, -2, 5, 1], 2) == Some(6).
pub fn max_window_sum(values: &[i32], k: usize) -> Option<i32> {
    if k == 0 || values.len() < k {
        return None;
    }

    let mut largest: Option<i32> = None;
    for window in values.windows(k) {
        let sum: i32 = window.iter().sum();
        largest = match largest {
            Some(best) if best >= sum => Some(best),
            _ => Some(sum),
        };
    }
    largest
}

/// Remove every element equal to `target`, in place. Returns how many were removed.
pub fn remove_all(values: &mut Vec<i32>, target: i32) -> usize {
    let len_before = values.len();
    values.retain(|x| *x != target);
    len_before - values.len()
}

/// The `n` largest values, from largest to smallest. Fewer if the slice is shorter.
/// Don't modify `values`. top_n(&[3, 9, 1, 9], 3) == [9, 9, 3].
pub fn top_n(values: &[i32], n: usize) -> Vec<i32> {
    let mut vec = values.to_vec();
    vec.sort_by_key(|x| std::cmp::Reverse(*x));
    vec.truncate(n);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_unique_values() {
        let data = [5, 3, 9, 3, 5, 5];
        assert_eq!(sorted_unique(&data), [3, 5, 9]);
        assert_eq!(data, [5, 3, 9, 3, 5, 5]); // unchanged
        assert_eq!(sorted_unique(&[]), []);
        assert_eq!(sorted_unique(&[7]), [7]);
    }

    #[test]
    fn median_values() {
        assert_eq!(median(&[3, 1, 2]), Some(2.0));
        assert_eq!(median(&[4, 1, 2, 3]), Some(2.5)); // (2 + 3) / 2
        assert_eq!(median(&[7]), Some(7.0));
        assert_eq!(median(&[]), None);
        let data = [3, 1, 2];
        median(&data);
        assert_eq!(data, [3, 1, 2]); // unchanged
    }

    #[test]
    fn chunk_sums_values() {
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 2), [3, 7, 5]);
        assert_eq!(chunk_sums(&[1, 2, 3], 3), [6]);
        assert_eq!(chunk_sums(&[1, 2, 3], 10), [6]);
        assert_eq!(chunk_sums(&[], 2), []);
        assert_eq!(chunk_sums(&[1, 2], 0), []); // size 0: no groups, and no panic
    }

    #[test]
    fn max_window_sum_values() {
        assert_eq!(max_window_sum(&[1, -2, 5, 1], 2), Some(6));
        assert_eq!(max_window_sum(&[1, -2, 5, 1], 1), Some(5));
        assert_eq!(max_window_sum(&[1, -2, 5, 1], 4), Some(5));
        assert_eq!(max_window_sum(&[1, 5, 5, 1], 2), Some(10)); // windows OVERLAP: [5, 5] counts
        assert_eq!(max_window_sum(&[-5, -3, -9], 1), Some(-3)); // all negative: the answer is < 0
        assert_eq!(max_window_sum(&[1, 2], 3), None);
        assert_eq!(max_window_sum(&[1, 2], 0), None);
        assert_eq!(max_window_sum(&[], 1), None);
    }

    #[test]
    fn remove_all_values() {
        let mut v = vec![1, 0, 2, 0, 0, 3];
        assert_eq!(remove_all(&mut v, 0), 3);
        assert_eq!(v, [1, 2, 3]);
        assert_eq!(remove_all(&mut v, 9), 0);
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn top_n_values() {
        assert_eq!(top_n(&[3, 9, 1, 9], 3), [9, 9, 3]);
        assert_eq!(top_n(&[3, 9, 1], 10), [9, 3, 1]);
        assert_eq!(top_n(&[3, 9, 1], 0), []);
        assert_eq!(top_n(&[], 3), []);
    }
}
