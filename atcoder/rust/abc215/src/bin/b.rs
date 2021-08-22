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
        n: u128,
    }

    let mut k = 0;
    while 2_u128.pow(k + 1) <= n {
        k += 1;
    }

    println!("{}", k)
}
