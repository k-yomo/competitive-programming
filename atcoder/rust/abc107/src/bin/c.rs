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
        x: [i64; n],
    }

    println!("{}",
             x.windows(k)
                 .map(|w| std::cmp::min(w[0].abs() + (w[k-1] - w[0]).abs(), w[k-1].abs() + (w[k-1] - w[0]).abs()))
                 .min().unwrap()
    );
}

