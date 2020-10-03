#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
     a: i64,   
     b: i64,   
     c: i64,   
     k: u32,   
    }

    let max_num = max(max(a, b), c);
    println!("{}", max_num * 2_i64.pow(k) + a + b + c - max_num);
}
