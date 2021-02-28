#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        (n, m): (usize, usize),
    }
    println!("{}", (1900 * m + 100 * (n - m)) * 2_usize.pow(m as u32));
}
