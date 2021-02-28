#![allow(unused_imports)]

use std::cmp::*;
use std::cmp::Ordering::{Greater, Less};
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;

fn main() {
    input! {
        (n,m,k): (usize, usize, usize),
        a: [usize; n],
        b: [usize; m],
    }
    let mut a_acc: Vec<usize> = once(0).chain(a).cumsum().filter(|&sum| { sum <= k }).collect();
    let mut b_acc: Vec<usize> = once(0).chain(b).cumsum().collect();
    let max_count = a_acc.iter().enumerate().map(|(i, a_sum)|  i + b_acc.upper_bound(&(k - a_sum)) - 1).max().unwrap();

    println!("{:?}", max_count);
}