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
        n: usize,
        min_maxes: [(usize, usize); n],
    }
    let mut count = 0;
    for (min, max) in min_maxes {
        count += max * (max + 1) / 2 - min * (min - 1) / 2;
    }

    println!("{}", count);
}
