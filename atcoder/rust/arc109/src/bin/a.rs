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
        a: i64, b: i64, x: i64, y: i64,
    }

    let mut min_time = (b - a).abs() * y + x;
    if a < b {
        min_time = std::cmp::min(min_time, (b - a) * x * 2 + x);
    } else if a > b {
        min_time = std::cmp::min(min_time, (a - b) * x * 2 - x);
    }

    println!("{}", std::cmp::min((2 * b + 1 - 2 * a).abs()),);
}
