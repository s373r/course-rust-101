// Rust-101, Part 10: Closures
// ===========================

use part05::BigInt;
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::Add;

// So, let us define a trait that demands that the type provides some method `do_action` on digits.
trait Action {
    fn do_action(&mut self, digit: u64);
}

// Now we can write a function that takes some `a` of a type `A` such that we can call `do_action`
// on `a`, passing it every digit.
impl BigInt {
    fn act_v1<A: Action>(&self, mut a: A) {
        for digit in self {
            a.do_action(digit);
        }
    }
}

struct PrintWithString {
    prefix: String,
}

impl Action for PrintWithString {
    // Here we perform the actual printing of the prefix and the digit. We're not making use of our
    // ability to change `self` here, but we could replace the prefix if we wanted.
    fn do_action(&mut self, digit: u64) {
        println!("{}{}", self.prefix, digit);
    }
}

// Finally, this function takes a `BigInt` and a prefix, and prints the digits with the given prefix.
fn print_with_prefix_v1(b: &BigInt, prefix: String) {
    let my_action = PrintWithString { prefix };
    b.act_v1(my_action);
}

// Here's a small main function, demonstrating the code above in action. Remember to edit `main.rs`
// to run it.
pub fn main() {
    let bignum = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    print_with_prefix_v1(&bignum, "Digit: ".to_string());
}

// ## Closures

// This defines `act` very similar to above, but now we demand `A` to be the type of a closure that
// mutates its borrowed environment, takes a digit, and returns nothing.
impl BigInt {
    fn act<A: FnMut(u64)>(&self, mut a: A) {
        for digit in self {
            // We can call closures as if they were functions - but really, what's happening here
            // is translated to essentially what we wrote above, in `act_v1`.
            a(digit);
        }
    }
}

// Now that we saw how to write a function that operates on closures, let's see how to write a
// closure.
pub fn print_with_prefix(b: &BigInt, prefix: String) {
    b.act(|digit| println!("{}{}", prefix, digit));
}
// You can change `main` to call this function, and you should notice - nothing, no difference in
// behavior. But we wrote much less boilerplate code!

// Remember that we decided to use the `FnMut` trait above? This means our closure could actually
// mutate its environment. For example, we can use that to count the digits as they are printed.
pub fn print_and_count(b: &BigInt) {
    let mut count: usize = 0;
    b.act(|digit| {
        println!("{}: {}", count, digit);
        count = count + 1;
    });
    println!("There are {} digits", count);
}

// ## Fun with iterators and closures

// Let's say we want to write a function that increments every entry of a `Vec` by some number,
// then looks for numbers larger than some threshold, and prints them.
fn inc_print_threshold(v: &Vec<i32>, offset: i32, threshold: i32) {
    for i in v.iter().map(|n| *n + offset).filter(|n| *n > threshold) {
        println!("{}", i);
    }
}

// Sometimes it is useful to know both the position of some element in a list, and its value.
// That's where the `enumerate` function helps.
fn print_enumerated<T: fmt::Display>(v: &Vec<T>) {
    for (i, t) in v.iter().enumerate() {
        println!("Position {}: {}", i, t);
    }
}

// And as a final example, one can also collect all elements of an iterator, and put them, e.g., in a vector.
fn filter_vec_by_divisor(v: &Vec<i32>, divisor: i32) -> Vec<i32> {
    v.iter().map(|n| *n).filter(|n| *n % divisor == 0).collect()
}

// **Exercise 10.1**: Look up the
// [documentation of `Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
// to learn about more functions that can act on iterators. Try using some of them. What about a
// function that sums the even numbers of an iterator? Or a function that computes the product of
// those numbers that sit at odd positions? A function that checks whether a vector contains a
// certain number? Whether all numbers are smaller than some threshold? Be creative!

fn sum_even_numbers<'a, T: Sum<&'a T> + PartialEq>(v: &'a Vec<T>) -> T {
    v.iter()
        .enumerate()
        .filter(|(i, x)| (i + 1) & 1 == 0)
        .map(|(_, x)| x)
        .sum()
}

fn product_odd_numbers<'a, T: Product<&'a T> + PartialEq>(v: &'a Vec<T>) -> T {
    v.iter()
        .enumerate()
        .filter(|(i, x)| (i + 1) & 1 == 1)
        .map(|(_, x)| x)
        .product()
}

#[cfg(test)]
mod tests {
    use part05::BigInt;
    use part10::{product_odd_numbers, sum_even_numbers, vec_min_with_map};

    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers(&vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(sum_even_numbers(&vec![1.1, 2.2, 3.1, 4.2, 5.1]), 6.4);
    }

    #[test]
    fn test_product_odd_numbers() {
        assert_eq!(product_odd_numbers(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(product_odd_numbers(&vec![1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
    }

    #[test]
    fn test_vec_min_with_map_or() {
        assert_eq!(vec_min_with_map(&vec![]), None);
        assert_eq!(vec_min_with_map(&vec![1, 2, 3, 4, 5]), Some(1));
        assert_eq!(vec_min_with_map(&vec![2, 2, 2]), Some(2));
        assert_eq!(vec_min_with_map(&vec![1, 2, 3, 4, 5, 0]), Some(0));
    }

    #[test]
    fn test_big_int_test_invariant_with_map_or() {
        let big_int = BigInt::from_vec_without_remove_last_zeroes(vec![1, 2]);

        assert_eq!(big_int.test_invariant_with_map_or(), true);

        let invalid_big_int = BigInt::from_vec_without_remove_last_zeroes(vec![1, 0]);

        assert_eq!(invalid_big_int.test_invariant_with_map_or(), false);
    }
}
// **Exercise 10.2**: We started the journey in Part 02 with `SomethingOrNothing<T>`, and later
// learned about `Option<T>` in Part 04. `Option<T>` also has a `map` function.
// [Read its documentation here.](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.map)
// Which functions in previous parts can you rewrite to use `map` instead?
// (Hint: read the source code of `map`, and see if the pattern appears in your own code.)
// Bonus: [`test_invariant` in Part 05](part05.html#section-6) doesn't use `match`,
// but can you still find a way to rewrite it with `map`?

fn vec_min_with_map(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;
    for e in v.iter() {
        // NOTE(DP): before
        // min = Some(match min {
        //     None => *e,
        //     Some(n) => cmp::min(n, *e),
        // });

        // NOTE(DP): after
        // min = min.map_or(*e, |n| cmp::min(n, *e));
        min = min.map(|n| cmp::min(n, *e)).or(Some(*e));
    }
    min
}

impl BigInt {
    fn test_invariant_with_map_or(&self) -> bool {
        // NOTE(DP): before
        // match self.data.last() {
        //     None => true,
        //     Some(last_digit) => *last_digit != 0,
        // }

        // NOTE(DP): after
        self.data.last().map_or(true, |last_digit| *last_digit != 0)
    }

    // NOTE(DP): just for unit-tests
    fn from_vec_without_remove_last_zeroes(v: Vec<u64>) -> Self {
        BigInt { data: v }
    }
}
