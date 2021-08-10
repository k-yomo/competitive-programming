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
        m: usize, n: usize, mut N: usize,
    }

    let mut sold_count = N;
    while N >= m {
        sold_count += (N / m) * n;
        N = (N / m) * n + N % m;
    }

    println!("{}", sold_count)
}
