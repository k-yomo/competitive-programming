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
        n: usize,
        a: [f64; n]
    }

    let mut diff = 0.0;
    for i in 0..n-1 {
        diff += a[i + 1] - a[i]
    }

    println!("{}", diff / (n as f64 - 1.0))
}
