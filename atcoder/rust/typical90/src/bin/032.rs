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
        a: usize, b: usize,
    }

    println!(lcm(a, b));
}
