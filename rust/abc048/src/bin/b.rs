#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    println!("{}", if a == 0 { b / x + 1 } else { b /x - (a-1) / x });
}
