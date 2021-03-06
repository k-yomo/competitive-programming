#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
    }

    let mut sum: f64 = 0.0;
    for i in 1..n {
        sum += 1.0 / (i as f64)
    }
    println!("{}", (n as f64) * sum);
}
