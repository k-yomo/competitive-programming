#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        mut n: f64,
        m: f64,
    }

    if n > 12.0 {
        n -= 12.0;
    }
    let little_hand = 30.0 * n + 0.5 * m;
    let big_hand = 6.0 * m;

    let mut min = (little_hand-big_hand).abs();
    if min > 180.0 {
        min = (360.0 - min).abs();
    }
    println!("{}", min);
}
