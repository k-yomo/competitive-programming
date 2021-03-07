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
        t: usize,
        lr: [(i64, i64); t],
    }

    for (l, r) in lr {
        let i = r - l * 2;
        if i < 0 {
            println!("0")
        } else {
            println!("{}", (i + 1) * (i + 2) / 2)
        }
    }
}
