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
        n: usize, W: usize,
        wv: [(usize, usize); n],
    }

    // dp[i][w] := i+1番目までの品物から重さが w を超えないように選んだときの、価値の総和の最大値
    let mut dp = vec![vec![0; W + 1]; n + 1];
    for (i, w) in iproduct!(0..n, 0..=W) {
        if wv[i].0 > w {
            dp[i + 1][w] = dp[i][w]
        } else {
            dp[i + 1][w] = std::cmp::max(dp[i][w], wv[i].1 + dp[i][w - wv[i].0])
        }
    }

    println!("{}", dp[n][W]);
}
