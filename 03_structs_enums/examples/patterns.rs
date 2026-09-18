// Run: cargo run --example patterns

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // ---------- literals, ranges, @ bindings: no guards, no `_` needed ----------
    for n in [i32::MIN, -7, 0, 5, 42, 1000] {
        let label = match n {
            ..0 => "negative".to_string(),
            0 => "zero".to_string(),
            d @ 1..=9 => format!("digit {d}"),
            10..100 => "two digits".to_string(),
            100.. => "large".to_string(),
        };
        println!("{n:>11}: {label}");
    }

    // ---------- tuples ----------
    for pair in [(0, 0), (3, 0), (0, -2), (4, 4), (1, 2)] {
        let s = match pair {
            (0, 0) => "origin".to_string(),
            (x, 0) | (0, x) => format!("on an axis at {x}"),
            (x, y) if x == y => "on the diagonal".to_string(), // a guard for what patterns can't say
            (x, y) => format!("at ({x}, {y})"),
        };
        println!("{pair:?}: {s}");
    }

    // ---------- structs: destructure, match on fields ----------
    let p = Point { x: 3, y: 0 };
    match p {
        Point { x: 0, y: 0 } => println!("origin"),
        Point { x, y: 0 } => println!("on the x axis at {x}"),
        Point { y, .. } => println!("somewhere with y={y}"),
    }
    let Point { x, y } = p; // let destructuring (i32 fields are Copy)
    println!("x={x} y={y}");

    // ---------- slices ----------
    for v in [&[][..], &[7], &[1, 2], &[1, 2, 3, 4]] {
        let s = match v {
            [] => "empty".to_string(),
            [one] => format!("just {one}"),
            [first, second] => format!("pair {first},{second}"),
            [first, .., last] => format!("{first} … {last} ({} items)", v.len()),
        };
        println!("{v:?}: {s}");
    }
    let words = ["cmd", "arg1", "arg2"];
    if let [command, args @ ..] = &words[..] {
        println!("command={command} args={args:?}");
    }

    // ---------- nested patterns ----------
    let maybe_point: Option<Point> = Some(Point { x: 0, y: 5 });
    if let Some(Point { x: 0, y }) = &maybe_point {
        println!("on the y axis at {y}");
    }

    // ---------- matching through a reference ----------
    let name: Option<String> = Some(String::from("neo"));
    let len = match &name {
        Some(n) => n.len(), // n: &String, nothing moved
        None => 0,
    };
    println!("len={len}, name still usable: {name:?}");

    // ---------- if let / let chains / let else ----------
    let input = "port=8080";
    if let Some((key, value)) = input.split_once('=')
        && key == "port"
    {
        println!("port value: {value}");
    }
    println!("port_or_default(\"localhost:8080\")={}", port_or_default("localhost:8080"));
    println!("port_or_default(\"localhost\")={}", port_or_default("localhost"));

    // ---------- while let ----------
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!("← popped until empty");

    // ---------- matches! ----------
    let c = 'e';
    println!("is a vowel: {}", matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));
}

// let-else: bind on the happy path, bail out otherwise.
// (In a function returning Option, `?` is shorter for "otherwise return None".)
fn port_or_default(addr: &str) -> u16 {
    let Some((_host, port)) = addr.split_once(':') else {
        return 80; // the else block must leave: return / break / continue / panic
    };
    port.parse().unwrap_or(80) // parse() returns a Result (Section 04); it has unwrap_or too
}
