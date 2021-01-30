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
        a: usize, b: usize, c: usize,
    }

    if c == 0 {
        println!("{}", if a > b { "Takahashi" } else { "Aoki" })
    } else {
        println!("{}", if b > a { "Aoki" } else { "Takahashi" })
    }
}
