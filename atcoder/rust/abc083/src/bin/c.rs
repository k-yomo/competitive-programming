#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
       mut x: usize,
       y: usize,
    }

    let mut count = 1;
    while x <= y {
        if x * 2 <= y {
            count += 1;
        }
        x *= 2;
    }

    println!("{}", count);
}
