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
        n: usize,
        t: [usize; n],
    }

    // dp[i+1][j] := i番目までの整数の中からいくつか選んで総和を jにすることができるか
    let mut s = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; s + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 0..n {
        for j in 0..=s {
            dp[i + 1][j] |= dp[i][j];
            if j >= t[i] {
                dp[i + 1][j] |= dp[i][j - t[i]]
            }
        }
    }

    let mut min_diff = std::usize::MAX;
    let mut m = 0;
    for j in 0..s {
        if !dp[n][j] {
            continue;
        }
        if s > j * 2 {
            if s - j * 2 < min_diff {
                min_diff = s - j * 2;
                m = s - j;
            }
        } else {
            if j * 2 - s < min_diff {
                min_diff = j * 2 - s;
                m = j;
            }
        }
    }
    println!("{}", m)
}
