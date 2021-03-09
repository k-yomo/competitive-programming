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
    mut b: [usize; n-1],
    }

    let mut members = vec![vec![]; n];
    for (i, b) in b.iter().enumerate() {
        members[b - 1].push(i + 1);
    }

    println!("{}", calc_salary(&members, 0));
}

fn calc_salary(members: &Vec<Vec<usize>>, i: usize) -> usize {
    if members[i].len() == 0 {
        return 1;
    }
    let mut salaries = members[i]
        .iter()
        .map(|m| calc_salary(&members, *m))
        .collect::<Vec<usize>>();
    return salaries.iter().max().unwrap() + salaries.iter().min().unwrap() + 1;
}
