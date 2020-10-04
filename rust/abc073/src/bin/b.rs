#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,        
        seats: [(usize, usize); n],
    }

    let mut sum = 0;
    for (l, r) in seats {
        sum += r-l+1;
    }
    println!("{}", sum);
}
