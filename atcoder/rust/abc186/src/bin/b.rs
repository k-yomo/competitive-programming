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
        (h, w): (usize, usize),
        a: [[usize; w]; h],
    }

    let min_block = iproduct!(0..h, 0..w).map(|(i, j)| a[i][j] ).min().unwrap();
    println!("{}", iproduct!(0..h, 0..w).map(|(i, j)| a[i][j] - min_block ).sum::<usize>());
}
