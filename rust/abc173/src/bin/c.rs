#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input!(h: usize, w: usize, k: usize, c: [Chars; h]);

    let count = iproduct!(0..1 << h, 0..1 << w ).filter(|&(row, col)| {
      iproduct!(
        (0..h).filter(|&i| row >> i & 1 == 1),
        (0..w).filter(|&j| col >> j & 1 == 1)
      ).filter(|&(i, j)| c[i][j] == '#').count() == k
    }).count();
    println!("{}", count);
}
