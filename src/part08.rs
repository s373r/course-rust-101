// Rust-101, Part 08: Associated Types, Modules
// ============================================

use part05::{remove_last_zeroes, BigInt};
use std::{cmp, ops};

// So, let us write a function to "add with carry", and give it the appropriate type. Notice Rust's
// native support for pairs.
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = a.wrapping_add(b);
    // If an overflow happened, then the sum will be smaller than *both* summands. Without an
    // overflow, of course, it will be at least as large as both of them. So, let's just pick one
    // and check.
    if sum >= a {
        // The addition did not overflow. <br/>
        // **Exercise 08.1**: Write the code to handle adding the carry in this case.
        let sum_with_carry = sum.wrapping_add(if carry { 1 } else { 0 });
        let has_overflow = if sum_with_carry >= sum { false } else { true };

        (sum_with_carry, has_overflow)
    } else {
        // Otherwise, the addition *did* overflow. It is impossible for the addition of the carry
        // to overflow again, as we are just adding 0 or 1.
        (sum + if carry { 1 } else { 0 }, true)
    }
}

// `overflow_add` is a sufficiently intricate function that a test case is justified.
// This should also help you to check your solution of the exercise.
#[test]
fn test_overflowing_add() {
    assert_eq!(overflowing_add(10, 100, false), (110, false));
    assert_eq!(overflowing_add(10, 100, true), (111, false));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
    assert_eq!(overflowing_add(1 << 63, (1 << 63) - 1, true), (0, true));
}

// ## Associated Types
impl ops::Add<BigInt> for BigInt {
    // Here, we choose the result type to be again `BigInt`.
    type Output = BigInt;

    // Now we can write the actual function performing the addition.
    fn add(self, rhs: BigInt) -> Self::Output {
        // We know that the result will be *at least* as long as the longer of the two operands,
        // so we can create a vector with sufficient capacity to avoid expensive reallocations.
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec: Vec<u64> = Vec::with_capacity(max_len);
        let mut carry = false; /* the current carry bit */

        for i in 0..max_len {
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);

            result_vec.push(sum);
            carry = new_carry;
        }

        // **Exercise 08.2**: Handle the final `carry`, and return the sum.
        if carry {
            result_vec.push(1);
        }

        BigInt::from_vec(result_vec)
    }
}

// ## Traits and reference types

// Writing this out becomes a bit tedious, because trait implementations (unlike functions) require
// full explicit annotation of lifetimes. Make sure you understand exactly what the following
// definition says. Notice that we can implement a trait for a reference type!
impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn add(self, rhs: &'a BigInt) -> Self::Output {
        // **Exercise 08.3**: Implement this function.
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec: Vec<u64> = Vec::with_capacity(max_len);
        let mut carry = false; /* the current carry bit */

        for i in 0..max_len {
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);

            result_vec.push(sum);
            carry = new_carry;
        }

        if carry {
            result_vec.push(1);
        }

        BigInt::from_vec(result_vec)
    }
}

// **Exercise 08.4**: Implement the two missing combinations of arguments for `Add`. You should not
// have to duplicate the implementation.

// NOTE(DP): I do not understand what is required :(

// ## Modules

// Rust calls a bunch of definitions that are grouped together a *module*. You can put the tests in
// a submodule as follows.
#[cfg(test)]
mod tests {
    use part05::BigInt;

    #[test]
    fn test_add() {
        let b1 = BigInt::new(1 << 32);
        let b2 = BigInt::from_vec(vec![0, 1]);
        let b3 = BigInt::from_vec(vec![1, 1]);
        let b4 = BigInt::new(1);
        let b5 = BigInt::from_vec(vec![u64::MAX]);

        assert_eq!(&b1 + &b2, BigInt::from_vec(vec![1 << 32, 1]));
        // **Exercise 08.5**: Add some more cases to this test.
        assert_eq!(&b2 + &b3, BigInt::from_vec(vec![1, 2]));
        assert_eq!(&b4 + &b5, BigInt::from_vec(vec![0, 1]));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            &BigInt::from_vec(vec![2, 3]) - &BigInt::from_vec(vec![1, 1]),
            Some(BigInt::from_vec(vec![1, 2]))
        );
        assert_eq!(
            &BigInt::from_vec(vec![5, 5]) - &BigInt::from_vec(vec![5, 5]),
            Some(BigInt::new(0))
        );
        assert_eq!(
            &BigInt::from_vec(vec![4, 6]) - &BigInt::from_vec(vec![0, 6]),
            Some(BigInt::new(4))
        );
        assert_eq!(
            &BigInt::from_vec(vec![1, 0, 1]) - &BigInt::new(1),
            Some(BigInt::from_vec(vec![0, 0, 1]))
        );
        assert_eq!(
            &BigInt::from_vec(vec![1, 0, 1]) - &BigInt::new(1),
            Some(BigInt::from_vec(vec![0, 0, 1]))
        );
    }
}

// **Exercise 08.6**: Write a subtraction function, and testcases for it. Decide for yourself how
// you want to handle negative results. For example, you may want to return an `Option`, to panic,
// or to return `0`.
impl<'a, 'b> ops::Sub<&'a BigInt> for &'b BigInt {
    type Output = Option<BigInt>;

    fn sub(self, rhs: &'a BigInt) -> Self::Output {
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec: Vec<u64> = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let left = if i < self.data.len() { self.data[i] } else { 0 };
            let right = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let delta = left.wrapping_sub(right);

            if left < delta {
                panic!("carry bit support is not implemented!");
            }

            result_vec.push(delta);
        }

        remove_last_zeroes(&mut result_vec);

        Some(if result_vec.is_empty() {
            BigInt::new(0)
        } else {
            BigInt::from_vec(result_vec)
        })
    }
}
