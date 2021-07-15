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
        s: Chars,
    }

    let mut count: usize = 0;
    let mut black_num: usize = 0;
    for c in s {
        if c == 'W' {
            count += black_num;
        } else {
            black_num += 1;
        }
    }

    println!("{}", count)
}
