#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        d: usize,
        x: usize,
        a: [usize; n],
    }

    println!("{}", n + a.iter().map(|i| (d-1)/i).sum::<usize>() + x)
}
