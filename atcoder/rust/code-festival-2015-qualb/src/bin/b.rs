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
        n: usize, m: usize,
        a: [usize; n],
    }

    let mut ans_counts = vec![0; m + 1];
    for i in a {
        ans_counts[i] += 1;
    }

    let max = ans_counts.iter().max().unwrap();
    let i = ans_counts.iter().position(|i| i == max).unwrap();
    if *max > n / 2 {
        println!("{}", i)
    } else {
        println!("?")
    }
}
