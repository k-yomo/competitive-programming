#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use num_integer::lcm;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().map(|num| num-1).sum::<usize>());
}
