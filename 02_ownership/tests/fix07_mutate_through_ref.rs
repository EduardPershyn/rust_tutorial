//! FIX 07: modifying the caller's data.
//! Make it compile and pass. Change the function and the call, not the asserts.
//! Run: cargo test --test fix07_mutate_through_ref

fn double_all(nums: &mut [i32]) {
    for n in nums {
        *n *= 2;
    }
}

#[test]
fn doubles_in_place() {
    let mut nums = vec![1, 2, 3];
    double_all(&mut nums);
    assert_eq!(nums, [2, 4, 6]);
}
