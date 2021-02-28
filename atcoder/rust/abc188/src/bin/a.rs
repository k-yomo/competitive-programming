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
        x: usize, y: usize,
    }

    println!("{}", if std::cmp::min(x, y) + 3 > std::cmp::max(x, y) { "Yes" } else { "No" });
}
