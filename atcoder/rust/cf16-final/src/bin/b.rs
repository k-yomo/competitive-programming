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
    }

    let mut sum = 0;
    let mut max = 0;
    let mut exclude = 0;
    for i in 1..=n {
        if sum + i < n {
            sum += i;
        } else {
            exclude = sum + i - n;
            max = i;
            break;
        }
    }
    for i in 1..=max {
        if i != exclude {
            println!("{}", i)
        }
    }
}
