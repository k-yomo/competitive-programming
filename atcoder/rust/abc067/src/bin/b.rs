#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        k: usize,
        mut l: [usize; n],
    }

    l.sort();
    println!("{}", l.iter().rev().take(k).sum::<usize>());
}
