#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        (n, m): (usize, usize),
    }

    if n >= m/2 {
        println!("{}", m/2);
        return
    }

    println!("{}", n + ((m - n * 2) / 4));
}
