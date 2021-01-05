#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut total_count: usize = 0;
    let mut cur = std::usize::MAX;
    let mut cur_len = 0;
    for num in a {
        if num > cur {
            cur_len += 1;
        } else {
            cur_len = 1;
        }
        total_count += cur_len;
        cur = num;
    }

    println!("{}", total_count);
}
