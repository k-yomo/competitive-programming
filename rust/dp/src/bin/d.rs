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
        n: usize, W: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; W + 1]; n + 1];
    for (i, w) in iproduct!(0..n, 0..W+1) {
        if w < wv[i].0 {
            dp[i + 1][w] = dp[i][w];
        } else {
            dp[i + 1][w] = std::cmp::max(dp[i][w - wv[i].0] + wv[i].1, dp[i][w]);
        }
    }

    println!("{}", dp[n][W]);
}
