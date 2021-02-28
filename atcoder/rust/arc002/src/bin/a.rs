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
        y: usize,
    }

    println!("{}", if y % 400 == 0 {
        "YES"
    } else if y % 100 == 0 {
        "NO"
    } else if y % 4 == 0 {
        "YES"
    } else {
        "NO"
    })
}
