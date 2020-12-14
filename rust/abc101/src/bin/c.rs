#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }

    println!("{}", ((n - k) + (k - 1) - 1) / (k - 1) + 1);
}

