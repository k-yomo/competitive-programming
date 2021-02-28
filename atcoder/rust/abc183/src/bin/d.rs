#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    }

    let limit = 2_000_001;
    let mut a: Vec<i64> = vec![0; limit];
    for (s, t, p) in stp {
        a[s] += p;
        a[t] -= p;
    }

    for i in 0..limit - 1 {
        a[i + 1] += a[i];
    }

    println!("{}", if a.iter().all(|&x| x <= w) { "Yes" } else { "No" });
}
