#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools::__std_iter::once;

fn main() { 
    input! {
        n: usize,
        mut h: [usize; n],
    }

    println!(
        "{}",
        once(0)
            .chain(h)
            .collect::<Vec<usize>>()
            .windows(2)
            .map(|w| if w[1] > w[0] { w[1] - w[0] } else { 0 })
            .sum::<usize>()
    );
}
