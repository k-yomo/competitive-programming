#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        (n, mut k): (usize, usize),
        ab: [(usize, usize); n],
    }
    for (a, b) in ab {
        if b < k {
            k -= b;
        } else {
            println!("{}", a);
            return;
        }
    }
}
