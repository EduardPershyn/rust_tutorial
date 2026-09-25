// Run: cargo run --example vectors

fn main() {
    // ---------- building and growing ----------
    let mut v = vec![3, 1, 4, 1, 5];
    v.push(9);
    v.extend([2, 6]);
    println!("v={v:?} len={} capacity={}", v.len(), v.capacity());

    let mut sized: Vec<i32> = Vec::with_capacity(100); // no reallocation for the first 100 pushes
    sized.push(1);
    println!("with_capacity: len={} capacity={}", sized.len(), sized.capacity());

    // watch the buffer grow: capacity doubles, and the data is copied to a new place
    let mut grow: Vec<u8> = Vec::new();
    let mut last_capacity = grow.capacity();
    for i in 0..20u8 {
        grow.push(i);
        if grow.capacity() != last_capacity {
            println!("  grew to capacity {} at len {}", grow.capacity(), grow.len());
            last_capacity = grow.capacity();
        }
    }

    // ---------- removing ----------
    let mut r = vec!["a", "b", "c", "d"];
    println!("remove(1)={} → {r:?}", r.remove(1)); // shifts the rest left: O(n)
    println!("swap_remove(0)={} → {r:?}", r.swap_remove(0)); // O(1), moves the last into the hole
    println!("pop()={:?} → {r:?}", r.pop());

    // ---------- sorting, dedup, search ----------
    let mut nums = vec![5, 3, 9, 1, 3, 5, 5];
    nums.sort();
    println!("sorted={nums:?}");
    nums.dedup(); // only removes CONSECUTIVE duplicates, hence the sort first
    println!("dedup={nums:?}");
    println!("binary_search(&5)={:?} binary_search(&4)={:?}", nums.binary_search(&5), nums.binary_search(&4));

    let mut words = vec!["hello", "hi", "greetings"];
    words.sort_by_key(|w| w.len()); // sort by a computed key (a closure: Section 08)
    println!("by length: {words:?}");

    let mut with_zeros = vec![0, 1, 0, 2, 3, 0];
    with_zeros.retain(|x| *x != 0); // keep what matches
    println!("retain non-zero: {with_zeros:?}");

    // ---------- windows and chunks ----------
    let data = [1, 2, 3, 4, 5];
    print!("windows(2): ");
    for w in data.windows(2) {
        print!("{w:?} ");
    }
    println!();
    print!("chunks(2): ");
    for c in data.chunks(2) {
        print!("{c:?} "); // the last chunk can be shorter
    }
    println!();

    // ---------- joining ----------
    let names = ["ann".to_string(), "bob".to_string()];
    println!("join: {}", names.join(", "));
    let nested = [vec![1, 2], vec![3], vec![4, 5]];
    println!("concat: {:?}", nested.concat());

    // ---------- collect: iterator → collection (Section 08) ----------
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    let as_text: Vec<String> = data.iter().map(|x| x.to_string()).collect();
    println!("doubled={doubled:?} as_text={}", as_text.join("+"));

    // ---------- a 2D grid ----------
    let mut grid = vec![vec![0; 3]; 2]; // 2 rows × 3 columns
    grid[1][2] = 7;
    println!("grid={grid:?}");
}
