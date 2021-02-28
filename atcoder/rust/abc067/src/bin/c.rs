#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let a_acc = a.iter().cumsum().collect::<Vec<i64>>();
    println!("{}", (0..n - 1).map(|i| (a_acc[i] - (a_acc[a_acc.len() - 1] - a_acc[i])).abs()).min().unwrap())
}
