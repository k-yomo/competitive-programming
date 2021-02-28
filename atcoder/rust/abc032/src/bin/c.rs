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
        (n, k): (usize, usize),
        s: [usize; n],
    }

    if { s.contains(&0) } {
        println!("{}", n);
        return;
    }

    let mut max_len = 0_usize;
    let (mut l, mut r, mut p) = (0, 0, 1);
    while l < n {
        while r < n && p * s[r] <= k {
            p *= s[r];
            r += 1;
        }
        max_len = std::cmp::max(max_len, r - l);
        if l == r {
            r += 1;
        } else {
            p /= s[l];
        }
        l += 1;
    }

    println!("{}", max_len);
}
