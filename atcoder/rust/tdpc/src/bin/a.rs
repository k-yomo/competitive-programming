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
        n: usize,
        p: [usize; n],
    }

    let mut dp = vec![100 * 100+ 1];
    dp[0] = true;
    for i in 0..n {
        dp[i][0] = true;
    }

    println!("{}", possible_score_map.keys().count());
}
