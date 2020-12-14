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
        (n, m): (usize, usize),
        mut x: [i64; m],
    }

    if n >= m {
        println!("0");
        return
    }

    x.sort();
    let mut diffs = x.windows(2).map(|w| { (w[1] - w[0]).abs() }).collect::<Vec<i64>>();
    diffs.sort();
    println!("{}", diffs[0..diffs.len()-(n-1)].iter().sum::<i64>());
}
