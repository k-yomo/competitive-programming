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
        n: usize,
        mut a: [i128; n],
    }

    let mut minus_count = a.iter().filter(|x| **x <= 0).count();
    if minus_count % 2 == 0 {
        println!("{}", a.iter().map(|x| x.abs()).sum::<i128>());
    } else {
        println!(
            "{}",
            a.iter().map(|x| x.abs()).sum::<i128>() - a.iter().map(|x| x.abs()).min().unwrap() * 2
        );
    }
}
