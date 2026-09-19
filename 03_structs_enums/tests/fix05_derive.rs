//! FIX 05: comparing and printing your own type.
//! `assert_eq!` needs `==` (PartialEq) and `{:?}` printing (Debug). `Point` has neither.
//! Make it compile by adding ONE line. Don't change the test.
//! Run: cargo test --test fix05_derive

#[derive(Debug, PartialEq)] 
struct Point {
    x: i32,
    y: i32,
}

fn mirror(p: Point) -> Point {
    Point { x: -p.x, y: p.y }
}

#[test]
fn mirrors() {
    assert_eq!(mirror(Point { x: 3, y: 4 }), Point { x: -3, y: 4 });
}
