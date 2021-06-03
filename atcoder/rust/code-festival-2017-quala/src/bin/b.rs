#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
    }

    for (y, x) in iproduct!(0..=m, 0..=n) {
        if k == y * (n - x) + x * (m - y) {
            return println!("Yes");
        }
    }
    println!("No")
}
