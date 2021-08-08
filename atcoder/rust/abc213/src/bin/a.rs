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
        a: usize,
        b: usize
    }

    for i in 0..=255 {
        if a ^ i == b {
            return println!("{}", i);
        }
    }
}
