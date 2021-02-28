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
        mut b: String,
    }

    let b: usize = b.replace(".", "").parse().unwrap();
    println!("{}", a * b / 100);
}
