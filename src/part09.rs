// Rust-101, Part 09: Iterators
// ============================

use part05::BigInt;

pub struct Iter<'a> {
    num: &'a BigInt,
    idx: usize, // the index of the last number that was returned
    end_idx: usize,
    stop: bool,
}

impl<'a> Iter<'a> {
    fn new(num: &'a BigInt) -> Self {
        Iter {
            num,
            idx: num.data.len() - 1,
            end_idx: 0,
            stop: false,
        }
    }

    fn new_rev(num: &'a BigInt) -> Self {
        Iter {
            num,
            idx: 0,
            end_idx: num.data.len() - 1,
            stop: false,
        }
    }
}

// Now we are equipped to implement `Iterator` for `Iter`.
impl<'a> Iterator for Iter<'a> {
    // We choose the type of things that we iterate over to be the type of digits, i.e., `u64`.
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.stop {
            return None;
        }

        let result = Some(self.num.data[self.idx]);

        self.stop = self.idx == self.end_idx;

        if !self.stop {
            let is_reverse_iter = self.end_idx != 0;

            if is_reverse_iter {
                self.idx += 1;
            } else {
                self.idx -= 1;
            }
        }

        result
    }
}

// All we need now is a function that creates such an iterator for a given `BigInt`.
impl BigInt {
    fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

// We are finally ready to iterate! Remember to edit `main.rs` to run this function.
pub fn main() {
    let b = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    for digit in b.iter() {
        println!("{}", digit);
    }
}

// Of course, we don't have to use `for` to apply the iterator. We can also explicitly call `next`.
fn print_digits_v1(b: &BigInt) {
    let mut iter = b.iter();

    loop {
        // Each time we go through the loop, we analyze the next element presented by the iterator
        // - until it stops.
        match iter.next() {
            None => break,
            Some(digit) => println!("{digit}"),
        }
    }
}

fn print_digits_v2(b: &BigInt) {
    let mut iter = b.iter();
    while let Some(digit) = iter.next() {
        println!("{}", digit);
    }
}

// **Exercise 09.1**: Write a testcase for the iterator, making sure it yields the corrects numbers.
#[cfg(test)]
mod tests {
    use part05::BigInt;

    #[test]
    fn test_big_int_iter() {
        let b = BigInt::from_vec(vec![3, 2, 1]);
        let mut iter = b.iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_big_int_iter_ldf() {
        let b = BigInt::from_vec(vec![3, 2, 1]);
        let mut iter = b.iter_ldf();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}

// **Exercise 09.2**: Write a function `iter_ldf` that iterates over the digits with the
// least-significant digits coming first. Write a testcase for it.

impl BigInt {
    fn iter_ldf(&self) -> Iter {
        Iter::new_rev(self)
    }
}

// ## Iterator invalidation and lifetimes

fn iter_invalidation_demo() {
    let mut b = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    for digit in b.iter() {
        println!("{}", digit);
        /*b = b + BigInt::new(1);*/ /* BAD! */
    }
}

// ## Iterator conversion trait

impl<'a> IntoIterator for &'a BigInt {
    type Item = u64;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}
// With this in place, you can now replace `b.iter()` in `main` by `&b`. Go ahead and try it! <br/>
