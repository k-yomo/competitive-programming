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
        n: usize, mut x: i128,
        a: [i128; n],
    }

    for (i, &v) in a.iter().enumerate() {
        x -= if i % 2 != 0 { v - 1 } else { v };
    }
    if x < 0 {
        return println!("No");
    }
    println!("Yes")
}
