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
        n: usize, q: usize,
        lr: [(usize, usize); q],
    }

    let mut flip_counts = vec![0_i64; n+1];
    for (l, r) in lr {
        flip_counts[l-1] += 1;
        flip_counts[r] -= 1;
    }

    let mut cur = 0;
    for i in 0..n {
        cur = (cur + flip_counts[i].abs()) % 2;
        print!("{}", cur);
    }
    println!("");
}
