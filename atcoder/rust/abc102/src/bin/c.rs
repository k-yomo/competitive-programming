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
        n: usize,
        mut a: [i128; n],
    }

    for i in 0..n {
        a[i] -= (i + 1) as i128;
    }
    a.sort();
    let b = a[n / 2];

    let mut min = 0;
    for (i, num) in a.iter().enumerate() {
        min += (num - b).abs();
    }

    println!("{}", min);
}
