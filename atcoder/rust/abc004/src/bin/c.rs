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

    let mut cards = vec![1, 2, 3, 4, 5, 6];
    for i in 0..n % 30 {
        cards.swap(i % 5, i % 5 + 1);
    }
    println!("{}", cards.iter().map(|x| { x.to_string() }).join(""))
}
