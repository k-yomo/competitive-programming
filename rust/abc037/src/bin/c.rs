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
        (n, k): (usize, usize),
        a: [usize; n],
    }

    let a_acc = once(0).chain(a).cumsum().collect::<Vec<usize>>();
    println!("{}", (k..(n + 1)).fold(0, |sum, i| sum + a_acc[i] - a_acc[i - k]));
}
