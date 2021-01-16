#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    let mut bh = BinaryHeap::new();
    for (i, (a, _)) in ab.iter().enumerate() {
        bh.push((Reverse(*a), i));
    }

    let mut min_time = 0;
    for _ in 0..k {
        let (Reverse(a), i) = bh.pop().unwrap();
        min_time += a;
        bh.push((Reverse(a + ab[i].1), i));
    }

    println!("{}", min_time);
}
