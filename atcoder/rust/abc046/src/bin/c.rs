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
        n: usize,
        ta: [(usize, usize); n],
    }

    let mut x = 1;
    let mut y = 1;
    for (t, a) in ta {
        let n = std::cmp::max(x / t, y / a);
        x *= n;
        y *= n;
    }
    println!("{}", x + y);
}
