// Run: cargo run --example moves
// Watch when the "drop(...)" lines appear: that's ownership in action.

// A type that prints a message when it is destroyed.
// (`struct` comes in Section 03 and `Drop` in Section 10; for now just watch the output.)
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("    drop({})", self.0);
    }
}

fn take(n: Noisy) {
    println!("  take() now owns {}", n.0);
} // n goes out of scope here → dropped

fn make(name: &'static str) -> Noisy {
    Noisy(name) // ownership moves out to the caller
}

fn main() {
    println!("1. leaving a scope drops its owners");
    {
        let _a = Noisy("a");
        println!("  inside the block");
    }
    println!("  after the block");

    println!("2. passing by value moves into the function");
    let b = Noisy("b");
    take(b);
    // println!("{}", b.0);       // ❌ E0382: borrow of moved value `b`
    println!("  back in main");

    println!("3. returning moves out of the function");
    let c = make("c");
    println!("  main now owns {}", c.0);

    println!("4. assignment moves; overwriting drops the old value");
    let mut d = Noisy("d1");
    let e = d; // d1 moves to e: no drop, just a new owner
    d = Noisy("d2"); // a moved-from variable can be given a new value
    println!("  e={}, d={}", e.0, d.0);
    d = Noisy("d3"); // d2 is dropped right here
    println!("  d={}", d.0);

    println!("5. String: move vs clone");
    let s1 = String::from("hello");
    let s2 = s1; // move: (ptr, len, cap) copied, heap untouched
    // println!("{s1}");          // ❌ E0382
    let s3 = s2.clone(); // deep copy: new heap buffer
    println!("  s2 heap at {:p}", s2.as_ptr());
    println!("  s3 heap at {:p}  ← different buffer", s3.as_ptr());
    let moved_again = s2;
    println!("  moved_again at {:p}  ← same buffer as s2", moved_again.as_ptr());

    println!("6. Copy types are copied, not moved");
    let x = 42;
    let y = x;
    let t = (1, 2.5, 'z');
    let t2 = t;
    println!("  x={x}, y={y}, t={t:?}, t2={t2:?}  ← all still usable");

    println!("7. a Vec owns its elements");
    let names = vec![String::from("ann"), String::from("bob")];
    // let first = names[0];      // ❌ E0507: cannot move out of index
    let first = &names[0]; // ✅ borrow it
    let second = names[1].clone(); // ✅ or copy it
    println!("  first={first}, second={second}");
    for name in names {
        // by-value loop: each String moves into `name`, and the Vec is consumed
        println!("  loop owns {name}");
    }
    // println!("{names:?}");     // ❌ E0382: the for loop consumed `names`

    println!("end of main: remaining owners are dropped in reverse order of declaration");
}
