#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        s: usize,
    }

    let mut dp = vec![0; s+1];
    dp[0] = 1;
    for i in 3..=s {
        for j in 0..=(i-3) {
            dp[i] += dp[j];
            dp[i] %= 1000000007;
        }
    }

    println!("{}", dp[s]);
}
