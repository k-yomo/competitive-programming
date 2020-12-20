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
        x: usize,
    }
    let mut sum = 0;
    for i in 1..10_usize.pow(9) {
        sum += i;
        if sum >= x {
            println!("{}", i);
            return
        }
    }
}
