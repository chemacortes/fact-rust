extern crate num;
use num::{One, BigUint};
use std::iter::FromIterator; // Necesitas importar FromIterator

pub fn fact(n: usize) -> BigUint {
    (1..=n)
        .map(BigUint::from)
        .product()
}
