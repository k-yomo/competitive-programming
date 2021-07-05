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
        q: usize, h: usize, s: usize, d: usize,
        n: usize,
    }

    let one_liter = std::cmp::min(std::cmp::min(q * 4, h * 2), s);

    if one_liter * 2 < d {
        println!("{}", n * one_liter)
    } else {
        println!("{}", n / 2 * d + (n % 2 * one_liter))
    }
}
