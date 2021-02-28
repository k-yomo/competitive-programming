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

    let mut statements = vec![];
    (0..n).for_each(|_| {
        input! { a: usize, xy: [(usize, usize); a] }
        statements.push(xy);
    });

    let max_honest_num = (0..(1 << n)).map(|flag| {
        (0..n).map(|i| if flag & 1 << i != 0 { 1 } else { 0 }).collect::<Vec<usize>>()
    })
        .filter(|comb| {
            (0..statements.len()).filter(|i| comb[*i] == 1).all(|i| statements[i].iter().all(|(x, y)| comb[x - 1] == *y))
        })
        .map(|comb| comb.iter().filter(|&&i| i == 1).count())
        .max().unwrap();

    println!("{}", max_honest_num);
}
