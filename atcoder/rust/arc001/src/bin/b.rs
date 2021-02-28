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
        mut a: i64, b: i64,
    }

    let mut count = 0;
    while a != b && count < 30 {
        let mut min_diff = 100;
        let mut min_diff_num = 0;
        for i in vec![-1, -5, -10, 1, 5, 10] {
            if (b - (a + i)).abs() < min_diff {
                min_diff = (b - (a + i)).abs();
                min_diff_num = i;
            }
        }
        a += min_diff_num;
        count += 1;
    }

    println!("{}", count);
}
