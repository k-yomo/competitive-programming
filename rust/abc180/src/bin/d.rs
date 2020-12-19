#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        (mut x, y, a, b): (u128, u128, u128, u128),
    }

    let mut exp = 0;
    while x * a <= x + b && x * a < y {
        x *= a;
        exp += 1;
    }
    println!("{}", exp + (y - 1 - x) / b);
}
