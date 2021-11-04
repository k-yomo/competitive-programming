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
use num_integer::lcm;

fn main() {
    input! {
        a: u128, b: u128,
    }

    let ans = lcm(a, b);
    if ans <= 10_u128.pow(18_u32) {
        println!("{}", ans)
    } else {
        println!("{}", "Large")
    };
}
