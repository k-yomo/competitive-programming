#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        (n, a, b): (usize, usize, usize),
        x: [usize; n],
    }


    let mut dp = vec![0; n];
    for i in 1..n {
        dp[i] = dp[i - 1] + std::cmp::min( (x[i] - x[i -1]) * a, b);
    }

    println!("{}", dp[n - 1]);
}
