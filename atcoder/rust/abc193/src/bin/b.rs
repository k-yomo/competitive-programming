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
        mut apx: [(f64, usize, f64); n],
    }

    apx.sort_by_key(|x| x.1);
    for (a, p, x) in apx {
        if x - a >= 1.0 {
            return println!("{}", p)
        }
    }
    println!("-1")
}
