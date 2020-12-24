#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        mut n: usize,
        a: [usize; n],
    }

    let (mut four_mul, mut two_mul, mut other): (usize, usize, usize) = (0, 0, 0);
    for num in a {
        if num % 4 == 0 {
            four_mul += 1;
        } else if num % 2 == 0 {
            two_mul += 1;
        } else {
            other += 1;
        }
    }
    if two_mul > 0 {
        other += 1;
    }
    println!("{}", if four_mul + 1 >= other { "Yes" } else { "No" });
}
