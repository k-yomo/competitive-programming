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
        amount: usize,
    }

    let mut charge = 1000 - amount;
    let coins = vec![500, 100, 50, 10, 5, 1];
    let mut coin_num = 0;
    for coin in coins {
        coin_num += charge / coin;
        charge = charge % coin;
    }

    println!("{}", coin_num);
}
