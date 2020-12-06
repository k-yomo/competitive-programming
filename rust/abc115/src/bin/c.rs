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
        mut h: [usize; n],
    }

    h.sort();
    println!("{}", (0..h.len()-(k-1)).map(|i|  &h[i+k-1] - &h[i]).min().unwrap());
}
