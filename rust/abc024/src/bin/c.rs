#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        (n, d, k): (usize, usize, usize),
        lr: [(usize, usize); d],
        st: [(usize, usize); k],
    }

    for (i, (s, t)) in st.iter().enumerate() {
        let mut cur = s;
        for (j, (l, r)) in lr.iter().enumerate() {
            if !(l <= cur && cur <= r) {
                continue
            }
            cur = if cur < t { std::cmp::min(r, t) } else { std::cmp::max(l, t) };
            if cur == t {
                println!("{}", j + 1);
                break
            }
        }
    }
}
