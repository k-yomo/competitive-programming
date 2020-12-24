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
        a: [i64; n],
    }

    println!("{}", (-100..101).map(|i| a.iter().map(|num| (i - num) * (i - num)).sum::<i64>()).min().unwrap());
}
