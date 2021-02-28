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
        h: [i64; n],
    }

    let mut dp = vec![0; n];
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        let mut min_step = std::i64::MAX;
        for j in 1..k + 1 {
            if i >= j {
                min_step = std::cmp::min(min_step, dp[i - j] + (h[i] - h[i - j]).abs());
            }
        }
        dp[i] = min_step;
    }

    println!("{}", dp[n - 1]);
}
