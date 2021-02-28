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
        xy: [(f64, f64); n],
    }

    let mut count = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let katamuki = (xy[j].1 - xy[i].1) / (xy[j].0 - xy[i].0);
            if katamuki >= -1.0 && katamuki <= 1.0 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
