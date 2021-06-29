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
        a: i64, b: i64,
    }

    let mut max_a = a;
    if a / 100 < 9 {
        max_a = 900 + a % 100;
    } else if a % 100 / 10 < 9 {
        max_a = a / 100 * 100 + 90 + a % 10;
    } else if a % 10 < 9 {
        max_a = a + (9 - a % 10);
    }

    let mut min_b = a;
    if b / 100 > 1 {
        min_b = 100 + b % 100;
    } else if b % 100 / 10 > 0 {
        min_b = b / 100 * 100 + b % 10;
    } else if a % 10 > 0 {
        min_b = b - b % 10;
    }

    println!("{}", std::cmp::max(max_a - b, a - min_b))
}
