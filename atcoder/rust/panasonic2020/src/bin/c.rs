#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    // √a + √b < √c
    // (√a + √b)**2 < c
    // (2√ab) < c - a - b
    // 4ab < (c - a - b)**2
    if c - a - b > 0 && 4*a*b < (c - a - b) * (c - a - b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
