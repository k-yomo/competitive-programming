#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        a: u128, b: u128, n: u128,
    }

    let x = std::cmp::min(b - 1, n);
    println!("{}", (a * x) / b - a * (x / b))
}
