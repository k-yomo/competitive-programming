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
        mut p: usize,
    }

    let mut coins = vec![3628800, 362880, 40320, 5040, 720, 120, 24, 6, 2, 1];
    let mut count = 0;
    for coin in coins {
        if p >= coin {
            let coin_count = std::cmp::min(p / coin, 100);
            p -= coin * coin_count;
            count += coin_count;
        }
    }

    println!("{}", count)
}
