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
    }

    println!("{}", if a > b * 2 { a - b * 2 } else { 0 });
}
