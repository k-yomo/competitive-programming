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
        n: usize,
        h: [i64; n],
    }

    let mut t = 0;
    let mut min_si = 0;
    let mut max_mt_size = 0;
    while t < n {
        if t < n - 1 && h[t + 1] > h[t] {
            t += 1;
            continue
        }
        let mut max_ui = t;
        while max_ui < n - 1 && h[max_ui + 1] < h[max_ui] {
            max_ui += 1;
        }
        max_mt_size = std::cmp::max(max_mt_size, max_ui - min_si + 1);
        min_si = max_ui;
        t = max_ui + 1;
    }

    println!("{}", max_mt_size);
}
