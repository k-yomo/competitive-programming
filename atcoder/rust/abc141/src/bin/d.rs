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
        n: usize, m: usize,
        mut a: [usize; n],
    }

    let mut bh = BinaryHeap::new();
    for i in a {
        bh.push(i);
    }

    for _ in 0..m {
        let price = bh.pop().unwrap();
        bh.push(price / 2);
    }

    println!("{}", bh.into_sorted_vec().iter().sum::<usize>());
}
