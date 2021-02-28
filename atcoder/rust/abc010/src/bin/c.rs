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
        (x1, y1, x2, y2, t, v): (i64, i64, i64, i64, f64, f64),
        n: usize,
        xy: [(i64, i64); n],
    }

    for (x, y) in xy {
        if (((x1 - x).pow(2) + (y1 - y).pow(2)) as f64).sqrt() + (((x - x2).pow(2) + (y - y2).pow(2)) as f64).sqrt() <= t * v {
            println!("YES");
            return
        }
    }
    println!("NO");
}
