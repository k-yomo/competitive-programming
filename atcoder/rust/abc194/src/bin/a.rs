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
        a: usize, b: usize,
    }
    println!(
        "{}",
        if (a + b) >= 15 && b >= 8 {
            1
        } else if (a + b) >= 10 && b >= 3 {
            2
        } else if (a + b) >= 3 {
            3
        } else {
            4
        }
    );
}
