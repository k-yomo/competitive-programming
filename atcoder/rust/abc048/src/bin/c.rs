#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize, x: usize,
        mut a: [usize; n],
    }

    let mut count = 0;
    for i in 1..n {
        if a[i - 1] + a[i] > x {
            let diff = a[i - 1] + a[i] - x;
            count += diff;
            a[i] -= std::cmp::min(diff, a[i]);
        }
    }

    println!("{}", count);
}
