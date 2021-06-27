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
        mut tlr: [(usize, usize, usize); n],
    }

    tlr.sort_by_key(|x| x.2);
    let mut count = 0;
    for i in 0..n - 1 {
        for j in (i + 1)..n {
            let (it, il, ir) = tlr[i];
            let (jt, jl, jr) = tlr[j];
            if ir < jl {
                continue;
            }
            if il == jl && ir == jr {
                count += 1;
                continue;
            }
            if ir <= jl && ((it == 2 || it == 4) || (jt >= 3)) {
                continue;
            }
            count += 1;
        }
    }

    println!("{}", count)
}
