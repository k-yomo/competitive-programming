#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        mut x: usize,
    }

    let mut ans = x / 11 * 2;
    x %= 11;
    if x > 0 {
        ans += if x > 6  { 2 } else { 1 };
    }

    println!("{}", ans);
}
