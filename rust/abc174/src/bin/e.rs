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
        (n, k): (usize, i64),
        mut a: [i64; n],
    }

    let (mut ok, mut ng) = (*a.iter().max().unwrap(), 0);
    let mut mid = (ok + ng) / 2;
    while ok - ng > 1 {
        if a.iter().map(|l| (l + mid - 1) / mid - 1).sum::<i64>() <= k {
            ok = mid;
        } else {
            ng = mid;
        }
        mid = (ok + ng) / 2;
    }

    println!("{}", ok);
}
