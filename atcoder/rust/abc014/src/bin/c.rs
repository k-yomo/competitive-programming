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
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut colors = vec![0; 1000002];
    for (a, b) in ab {
        colors[a] += 1;
        colors[b + 1] -= 1;
    }

    let mut cur = 0;
    let mut max = 0;
    for color in colors {
        cur += color;
        max = std::cmp::max(max, cur);
    }

    println!("{}", max);
}
