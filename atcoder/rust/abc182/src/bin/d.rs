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

fn main() {
    input! {
        n: usize,
        a: [i128; n],
    }

    let mut total_diff = 0;
    let mut max_diff = 0;
    let mut cur_x = 0;
    let mut max = 0;
    for x in a {
        total_diff += x;
        max_diff = std::cmp::max(max_diff, total_diff);
        max = std::cmp::max(max, cur_x + max_diff);
        cur_x += total_diff;
    }

    println!("{}", max);
}
