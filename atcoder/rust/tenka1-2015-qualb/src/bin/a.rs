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
    let mut dp = vec![100, 100, 200];
    for i in 3..20 {
        dp.push(dp[i - 1] + dp[i - 2] + dp[i - 3])
    }
    println!("{}", dp[19])
}
