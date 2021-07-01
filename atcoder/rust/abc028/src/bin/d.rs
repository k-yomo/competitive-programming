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
        n: f64, k: f64,
    }

    println!(
        "{}",
        ((k - 1.0) * (n - k) * 6.0 + (n - 1.0) * 3.0 + 1.0) / (n * n * n)
    )
}
