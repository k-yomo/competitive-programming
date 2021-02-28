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
        n: usize,
    }

    iproduct!(1..10, 1..10)
        .filter(|(i, j)| i * j == 2025 - n as i32)
        .for_each(|(i, j)| println!("{} x {}", i, j));
}

