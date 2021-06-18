#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        W: usize,
        n: usize, K: usize,
        ab: [(usize, usize); n],
    }

    // dp[i][k][w] := i番目までのスクショからk枚、幅がwを超えないように選んだときの、重要度の総和の最大値
    let mut dp = vec![vec![0; W + 1]; n + 1];
    for (i, w) in iproduct!(0..n, 0..=W) {}
}
