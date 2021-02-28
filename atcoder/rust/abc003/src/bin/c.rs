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
        mut r: [i64; n],
    }

    r.sort();

    let mut cur_rate = 0_f64;
    for rate in r[(r.len() - k)..].iter() {
        cur_rate = (cur_rate + *rate as f64) / 2.0;
    }
    println!("{}", cur_rate);
}
