#![no_main]

// Issue #15577
struct Weird<'a>(&'a i32);

impl Clone for Weird<'_> {
    fn clone(&self) -> Self {
        println!("clone() called");
        Weird(self.0)
        // *self
    }
}

impl Copy for Weird<'static> {}

fn main() {
    let local = 1;
    let _ = [Weird(&local)].clone();
}
