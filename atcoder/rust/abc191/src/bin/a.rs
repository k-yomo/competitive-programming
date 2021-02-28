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
        v: usize, t: usize, s: usize, d: usize,
    }

    println!(
        "{}",
        if v * t <= d && d <= v * s {
            "No"
        } else {
            "Yes"
        }
    );
}
