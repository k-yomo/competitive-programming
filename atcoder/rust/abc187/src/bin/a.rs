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
        (a, b): (usize, usize),
    }

    println!("{}", std::cmp::max(digit_sum(a), digit_sum(b)));
}

fn digit_sum(mut num: usize) -> usize {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
