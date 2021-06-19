#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize, k: usize, mut s: usize,
    }

    (0..k).for_each(|_| print!("{} ", s));
    for _ in 0..n - k {
        print!("{} ", if s == 1000000000 { 1 } else { 1000000000 })
    }
}
