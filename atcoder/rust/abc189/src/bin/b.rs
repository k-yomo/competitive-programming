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
        n: usize, x: u128,
        vp: [(u128, u128); n],
    }

    let mut cur = 0;
    for (i, (v, p)) in vp.iter().enumerate() {
        cur += v * p;
        if cur > x * 100 {
            return println!("{}", i + 1);
        }
    }

    println!("-1");
}
