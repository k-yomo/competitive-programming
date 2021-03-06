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
        a: [i128; n],
    }

    let a_acc = a.clone().iter().cumsum().collect::<Vec<i128>>();

    let mut sum = 0;
    let mut s = 0;
    for i in 0..n {
        sum += (a[i]).pow(2) * (n - 1) as i128;
        s += a[i] * (a_acc[n - 1] - a_acc[i]);
    }

    println!("{}", sum - 2 * s);
}
