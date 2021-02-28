#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use num::integer::gcd;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left = vec![a[0]; n];
    let mut right = vec![a[n - 1]; n];
    for i in 1..n {
        left[i] = gcd(left[i-1], a[i]);
    }
    for i in 1..n {
        right[i] = gcd(right[i-1], a[n - 1 - i]);
    }
    println!("{}", (0..n).map(|i| {
        if i == 0 { return right[n-2] }
        if i == n-1 { return left[n-2] }
        gcd(left[i-1], right[a.len() - 2 - i])
    }).max().unwrap());
}
