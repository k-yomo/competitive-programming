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
        n: usize,x: usize,
        ab: [(usize, usize); n],
    }

    let mut min_time = std::usize::MAX;
    let mut cur_time = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        cur_time += a;
        let play_time = (x-i) * b;
        if cur_time + play_time < min_time {
            min_time = cur_time + play_time;
        }
        cur_time += b;
    }
    println!("{}", min_time);
}
