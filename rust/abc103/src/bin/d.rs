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
        (n, m): (usize, usize),
        mut ab: [(usize, usize); m],
    }

    ab.sort_by(|a, b| a.1.cmp(&b.1));
    let mut last_removed_bridge = 0;
    let mut count = 0;
    for (a, b) in ab {
        if  a > last_removed_bridge {
            last_removed_bridge = b - 1;
            count += 1;
        }
    }

    println!("{}", count);
}
