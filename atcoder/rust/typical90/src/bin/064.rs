#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
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
        mut a: [i128; n],
        lrv: [(usize, usize, i128); q],
    }

    let mut total = 0;
    let mut diffs = vec![];
    for i in 1..a.len() {
        total += (a[i-1] - a[i]).abs();
        diffs[i] = (a[i-1] - a[i]).abs();
    }

    for (l, r, v) in lrv {
        let mut diff: i128 = 0;
        if l > 1 {
            diff += (diffs[l-2] + v).abs() - diffs[l-2];
            diffs[l-2] += v;
        }
        if r < n {
            diff += (diffs[r-1] - v).abs() - diffs[r-1];
            diffs[r-1] += v;
        }
        total += diff;
        seg_tree.apply_range(l-1, r-1, v);
    }
}
