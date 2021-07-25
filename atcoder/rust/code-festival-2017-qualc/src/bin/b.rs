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
        a: [usize; n],
    }

    let all = 3_usize.pow(n as u32);
    let mut odd_num = 1;
    for i in a {
        if i % 2 == 0 {
            odd_num *= 2
        }
    }
    println!("{}", all - odd_num)
}
