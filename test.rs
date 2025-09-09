#![no_main]

struct Weird<B>(B);

impl<B> Clone for Weird<B>
where
    B: Clone,
    B: Copy,
    B: Iterator,
{
    fn clone(&self) -> Self {
        println!("clone() called");
        Weird(self.0.clone())
    }
}

// impl Copy for Weird<i32> {}
