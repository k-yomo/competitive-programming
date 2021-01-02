#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        (mut n, m): (usize, usize),
        mut p: [usize; n],
    }
    n += 1;
    p.push(0);

    let mut s = vec![];
    for (i, j) in iproduct!(0..n, 0..n) {
        s.push(p[i] + p[j])
    }
    s.sort();

    let mut max_score = 0;
    for (i, j) in iproduct!(0..n, 0..n) {
        let mut sum = p[i] + p[j];
        if sum > m {
            continue;
        }
        max_score = std::cmp::max(max_score, sum + s[s.upper_bound(&(m - sum)) - 1]);
    }

    println!("{}", max_score);
}

