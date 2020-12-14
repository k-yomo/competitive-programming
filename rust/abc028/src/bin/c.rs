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
        nums: [usize; 5],
    }

    let mut combs = nums.iter().combinations(3).map(|nums| nums.into_iter().sum::<usize>()).collect::<Vec<usize>>();
    combs.sort();
    println!("{}", combs[combs.len()-3]);
}
