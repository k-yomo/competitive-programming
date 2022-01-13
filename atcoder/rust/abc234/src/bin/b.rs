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
        xy: [(f64, f64); n]
    }

    let mut max = 0.0;
    for i in 0..n -1 {
        let (x1, y1) = xy[i];
        for j in 0..n {
            let (x2, y2) = xy[j];
            let length = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
            if length > max {
                max = length
            }
        }
    }

    println!("{}", max)
}
