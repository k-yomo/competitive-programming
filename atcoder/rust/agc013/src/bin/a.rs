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
        a: [usize; n],
    }

    let mut is_increasing = false;
    let mut is_decreasing = false;
    let mut count = 1;
    for i in 1..n {
        if a[i] > a[i - 1] {
            is_increasing = true;
        } else if a[i] < a[i - 1] {
            is_decreasing = true;
        }
        if is_increasing && is_decreasing {
            count += 1;
            is_increasing = false;
            is_decreasing = false;
        }
    }

    println!("{}", count)
}
