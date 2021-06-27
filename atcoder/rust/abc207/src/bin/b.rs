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
        mut a: u128, b: u128, c: u128, d: u128,
    }

    let mut count = 0;
    let mut red_num = 0;
    while a > red_num * d {
        if count > 10000000 {
            return println!("-1");
        }
        count += 1;
        a += b;
        red_num += c;
    }
    println!("{}", count);
}
