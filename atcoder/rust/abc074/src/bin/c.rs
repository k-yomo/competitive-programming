#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize, b: usize, c: usize, d: usize, e: usize, f: usize,
    }

    let mut dp = vec![(0, 0, 0, 0); n + 1];
    for i in 1..n + 1 {
        let (last_a_max, last_b_max, last_c_max) = dp[i - 1];
        let (a, b, c) = abc[i - 1];
        dp[i].0 = std::cmp::max(last_b_max, last_c_max) + a;
        dp[i].1 = std::cmp::max(last_a_max, last_c_max) + b;
        dp[i].2 = std::cmp::max(last_a_max, last_b_max) + c;
    }

    println!("{}", std::cmp::max(std::cmp::max(dp[n].0, dp[n].1), dp[n].2))
}
