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
        n: usize,
        s: [String; n],
    }

    let mut dp: Vec<u128> = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] = if s[i - 1] == "AND" {
            dp[i - 1]
        } else {
            dp[i - 1] + 2_u128.pow(i as u32)
        }
    }

    println!("{}", dp[n])
}
