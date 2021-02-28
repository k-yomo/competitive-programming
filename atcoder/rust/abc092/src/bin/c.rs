#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.insert(0, 0);
    a.push(0);
    let cost = a.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<i64>();
    for i in 1..n+1 {
        let last = a[i - 1];
        let next = a[i + 1];
        println!("{}", cost + (next - last).abs() - (a[i] - last).abs() - (next - a[i]).abs())
    }
}
