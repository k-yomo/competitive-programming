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
        n: usize, a: f64, b: f64,
        mut s: [usize; n],
    }

    let max = *s.iter().max().unwrap() as f64;
    let min = *s.iter().min().unwrap() as f64;
    if max == min {
        return println!("-1");
    }
    let p = b / (max - min);
    let q = a - p * (s.iter().map(|&i| i as f64).sum::<f64>() / n as f64);
    println!("{} {}", p, q);
}
