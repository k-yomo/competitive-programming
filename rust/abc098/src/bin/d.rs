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
        a: [usize; n],
    }

    let mut count = 0_usize;
    let (mut l, mut r, mut xor, mut sum) = (0, 0, 0, 0);
    while l < n {
        while r < n && sum ^ a[r] == sum + a[r] {
            sum += a[r];
            r += 1;
        }
        count += r - l;
        if l == r {
            r += 1;
        } else {
            sum -= a[l];
        }
        l += 1;
    }

    println!("{}", count);
}

