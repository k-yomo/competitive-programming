#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        s: String,        
    }

    let mut x = 0;
    let mut max = 0;
    for c in s.chars() {
        if c == 'I' {
            x += 1;
        } else {
            x -= 1;
        }
        if max < x {
            max = x;
        }
    }
    println!("{}", max);
}
