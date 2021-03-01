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
        n: usize, m: usize,
        a: [usize; m],
    }

    let mut a_map: HashMap<usize, bool> = HashMap::new();
    for i in a {
        a_map.insert(i, true);
    }

    let mut dp = vec![0; n + 1];
    if !*a_map.entry(0).or_default() { dp[0] = 1 }
    if !*a_map.entry(1).or_default() { dp[1] = 1 }
    for i in 2..=n {
        if *a_map.entry(i).or_default() {
            continue
        }
        dp[i] = (dp[i - 1] + dp[i - 2]) % 1000000007;
    }

    println!("{}", dp[n]);
}
