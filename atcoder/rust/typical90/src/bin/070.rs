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
        n: usize,
        xy: [(i128, i128); n],
    }

    let fac_x = xy.iter().map(|(x, _)| x).sorted().nth(n / 2).unwrap();
    let fac_y = xy.iter().map(|(_, y)| y).sorted().nth(n / 2).unwrap();

    let mut total = 0;
    for (x, y) in xy.iter() {
        total += (x - fac_x).abs() + (y - fac_y).abs();
    }

    println!("{}", total);
}
