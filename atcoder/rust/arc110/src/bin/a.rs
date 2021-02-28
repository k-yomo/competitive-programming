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
    }
    println!("{}", (2..n+1).fold(2, |ans, i| lcm(ans, i))+1);
}
