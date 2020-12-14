#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use num_integer::gcd;

fn main() { 
    input! {
        n: usize,
        mut x: [usize; n+1],
    }
    x.sort();
    let diffs  = x.windows(2).map(|w| w[1] - w[0]).collect::<Vec<usize>>();
    println!("{}", diffs[1..].iter().fold(diffs[0], |ans, &num| gcd(ans, num)));
}
