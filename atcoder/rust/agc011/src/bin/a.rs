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
        n: usize, c: usize, k: usize,
        mut t: [usize; n],
    }

    t.sort();
    let mut bus_count = 1;
    let mut cur_count = 1;
    let mut limit = t[0] + k;
    for &i in t[1..].iter() {
        if i <= limit && cur_count < c {
            cur_count += 1;
            continue;
        } else {
            bus_count += 1;
            cur_count = 1;
            limit = i + k;
        }
    }
    println!("{}", bus_count);
}
