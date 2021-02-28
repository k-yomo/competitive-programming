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
        mut xl: [(i64, i64); n],
    }

    xl.sort_by(|a, b| (a.0 + a.1).cmp(&(&b.0 + &b.1)));
    let mut cur = std::i64::MIN;
    let mut remain_count = 0;
    for (x, l) in xl.iter() {
        if x - l < cur {
            continue
        }
        remain_count += 1;
        cur = x + l;
    }

    println!("{}", remain_count);
}
