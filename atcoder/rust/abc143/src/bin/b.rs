#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::Itertools;
 
fn main() { 
    input! {
        n: usize,
        d: [usize; n],
    }


    let mut sum = 0;
    for comb in (0..n).combinations(2) {
        sum += d[comb[0]] * d[comb[1]];
    }

    println!("{}", sum);
}
