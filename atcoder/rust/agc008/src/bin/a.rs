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
        x: i128, y: i128,
    }

    let ans = if x >= 0 && y > 0 || x < 0 && y < 0 {
        if x <= y {
            abs_diff(x, y)
        } else {
            abs_diff(x, y) + 2
        }
    } else {
        if x < 0 && y == 0 {
            x.abs()
        } else {
            abs_diff(x, y) + 1
        }
    };

    println!("{}", ans)
}

fn abs_diff(x: i128, y: i128) -> i128 {
    return (y.abs() - x.abs()).abs();
}
