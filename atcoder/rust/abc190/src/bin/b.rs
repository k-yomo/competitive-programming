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
        n: usize, s: usize, d: usize,
        xy: [(usize, usize); n],
    }

    for (x, y) in xy {
        if x < s && y > d {
            return println!("Yes")
        }
    }

    println!("No")
}
