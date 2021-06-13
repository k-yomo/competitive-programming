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
        n: usize, q: usize,
        a: [u128; n],
        k: [u128; q],
    }

    for i in k {
        let mut last_i: u128 = 0;
        let mut cur_num: u128 = i;
        loop {
            let cur_i = a.upper_bound(&cur_num) as u128;
            if cur_i == last_i {
                break;
            }
            cur_num += cur_i - last_i;
            last_i = cur_i;
        }
        println!("{}", cur_num)
    }
}
