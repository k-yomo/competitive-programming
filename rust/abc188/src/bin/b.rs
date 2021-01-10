#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut naiseki  = 0_i64;
    for i in 0..n {
        naiseki += a[i] * b[i];
    }
    println!("{}", if naiseki == 0{ "Yes" } else { "No" });
}
