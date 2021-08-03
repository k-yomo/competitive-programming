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
        mut a: u128, k: u128,
    }

    let want = 2 * 10_u128.pow(12_u32);

    if k == 0 {
        return println!("{}", want - a)
    }

    let mut count = 0;
    while a < want {
        a += 1 + a * k;
        count += 1;
    }

    println!("{}", count);
}
