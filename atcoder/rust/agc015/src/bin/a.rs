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
        n: usize, a: usize, b: usize,
    }
    if n == 1 {
        return println!("{}", if a == b { 1 } else { 0 });
    }
    if a > b {
        return println!("0");
    }

    println!("{}", (a + b * (n - 1)) - (b + a * (n - 1)) + 1)
}
