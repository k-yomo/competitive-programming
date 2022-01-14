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
        n: usize, T: u128,
        t: [u128; n],
    }

    let mut cur_time = t[0] + T;
    let mut total = T;
    for &i in t[1..].iter() {
        total += if cur_time >= i { (T - (cur_time - i)) } else { T };
        cur_time = i + T;
    }

    println!("{}", total)
}
