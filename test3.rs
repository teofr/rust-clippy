#![no_main]

struct Weird<'a>(&'a i32);

impl Clone for Weird<'_> {
    fn clone(&self) -> Self {
        Weird(self.0)
    }
}

impl Copy for Weird<'static> {}

impl Weird<'_> {
    fn foo(&self) -> Self {
        self.clone()
    }
}
