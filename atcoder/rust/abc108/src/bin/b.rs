#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        x1: i64,        
        y1: i64,        
        x2: i64,        
        y2: i64,        
    }

    let x = x2 - x1;
    let y = y2 - y1;
    println!("{} {} {} {}", x2 - y, y2 + x, x1 - y, y1 + x);
}
