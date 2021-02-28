#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        (n, t): (usize, usize),
        ts: [usize; n],
    }

    let mut total = t;
    for (i, &cur_time) in ts[1..].iter().enumerate() {
        if cur_time >= ts[i] + t {
            total += t;
        } else {
            total += cur_time - ts[i];
        }
    }

    println!("{}", total);
}
