// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        match self.data.last() {
            None => true,
            Some(last_digit) => *last_digit != 0,
        }
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into
    // a number, by removing trailing zeros. The `mut` declaration for `v` here is just like the
    // one in `let mut ...`: We completely own `v`, but Rust still asks us to make our intention of
    // modifying it explicit. This `mut` is *not* part of the type of `from_vec` - the caller has
    // to give up ownership of `v` anyway, so they don't care anymore what you do to it.
    //
    // **Exercise 05.1**: Implement this function.
    //
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        v.reverse();

        BigInt { data: v }
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0, 1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`.
use part02::{Nothing, Something, SomethingOrNothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the
// number of digits? The number of non-zero digits? The smallest/largest digit? Of course, these
// should all take `self` as a shared reference (i.e., in borrowed form).

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}

fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */
    /* BAD! */
    *ptr = 1337;
}
