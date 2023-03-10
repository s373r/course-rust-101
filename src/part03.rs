// Rust-101, Part 03: Input
// ========================

use std::fmt::Debug;
// I/O is provided by the module `std::io`, so we first have to import that with `use`.
// We also import the I/O *prelude*, which makes a bunch of commonly used I/O stuff
// directly available.
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn read_vec<T: FromStr>() -> Vec<T> {
    let mut vec = Vec::new();
    // The central handle to the standard input is made available by the function `io::stdin`.
    let stdin = io::stdin();

    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    for line in stdin.lock().lines() {
        // Rust's type for (dynamic, growable) strings is `String`. However, our variable `line`
        // here is not yet of that type: It has type `io::Result<String>`.

        // I chose the same name (`line`) for the new variable to ensure that I will never,
        // accidentally, access the "old" `line` again.
        let line = line.unwrap();
        // Now that we have our `String`, we want to make it an `i32`.

        match line.trim().parse() {
            Ok(num) => vec.push(num),
            // We don't care about the particular error, so we ignore it with a `_`.
            Err(_) => println!("A line [{line}] is not a number!"),
        }
    }

    vec
}

// For the rest of the code, we just re-use part 02 by importing it with `use`.
use part02::{vec_min, Nothing, Something, SomethingOrNothing};

// If you update your `main.rs` to use part 03, `cargo run` should now ask you for some numbers,
// and tell you the minimum. Neat, isn't it?
pub fn main() {
    let vec: Vec<f32> = read_vec();
    let min = vec_min(vec);

    min.print2();
}

// **Exercise 03.1**: Define a trait `Print` to write a generic version of
// `SomethingOrNothing::print`.
// Implement that trait for `i32`, and change the code above to use it.
// I will again provide a skeleton for this solution. It also shows how to attach bounds to generic
// implementations (just compare it to the `impl` block from the previous exercise).
// You can read this as "For all types `T` satisfying the `Print` trait, I provide an implementation
// for `SomethingOrNothing<T>`".
//
// Notice that I called the function on `SomethingOrNothing` `print2` to disambiguate from the
// `print` defined previously.
//
// *Hint*: There is a macro `print!` for printing without appending a newline.
pub trait Print {
    fn print2(self);
}

impl<T: Debug> Print for SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Something(value) => print!("Something({value:?})"),
            Nothing => print!("Nothing"),
        }
    }
}

// **Exercise 03.2**: Building on exercise 02.2, implement all the things you need on `f32` to make
// your program work with floating-point numbers.
