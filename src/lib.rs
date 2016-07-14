extern crate num;

use num::{ One, BigUint, range_inclusive };

pub fn fact(n: usize) -> BigUint {
    range_inclusive(BigUint::one(), BigUint::from(n))
      .fold(BigUint::one(), |res, x| x * res)
}

#[test]
fn test_fact5() {
  assert_eq!(fact(5), BigUint::from(120usize));
  println!("fact(5) correcto");
}

#[test]
#[ignore]
fn test_factlarge() {
  let s = fact(150000);
  assert_eq!(s.to_string().len(), 711273);
}


/* More implementations */

fn fact1(n: usize) -> BigUint {
    let mut res = BigUint::one();

    for i in 2..n + 1 {
        res = &res * BigUint::from(i);
    }
    res
}

fn fact2(n: usize) -> BigUint {
    (2..n + 1).fold(BigUint::one(), |x, y| x * BigUint::from(y))
}

fn fact3(n: usize) -> BigUint {
    let mut i   = BigUint::from(2u32);
    let mut res = BigUint::one();

    while &i <= &BigUint::from(n) {
        res = res * &i;
        i = i + BigUint::one();
    }
    res
}

fn fact4(n: u32) -> BigUint {
    let mut res: BigUint = BigUint::one();
    let mut i: u32 = 1;

    while i <= n {
        res = res * BigUint::from(i);
        i += 1;
    }
    res
}
