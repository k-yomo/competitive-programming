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
        (n, m, q): (usize, usize, usize),
        abcd: [(usize, usize, usize, usize); q],
    }

    let max_score = (1..m + 1).combinations_with_replacement(n).map(|comb| {
        abcd.iter().fold(0, |sum, (a,b,c,d)| {
            sum + if comb[*b-1] - comb[*a-1] == *c { d } else { &0 }
        })
    }).max().unwrap();

    println!("{}", max_score);
}
