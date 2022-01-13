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
        t: u128,
    }

    println!("{}",  f(f(f(t) + t) + f(f(t))))
}

fn f(num: u128) -> u128 {
    num * num + 2 * num + 3
}
