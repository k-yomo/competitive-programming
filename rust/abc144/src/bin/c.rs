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
    }

    let min_move_count = (1..n).take_while(|i| i * i <= n)
        .filter(|i| n % i == 0)
        .map(|i| i + n/i - 2)
        .min().unwrap();

    println!("{}", min_move_count);
}
