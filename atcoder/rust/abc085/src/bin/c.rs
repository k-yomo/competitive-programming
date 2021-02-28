#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        (n, y): (usize, usize),
    }
    for i in 0..20001 {
        if 10000 * i > y {
            break
        }
        for j in 0..n-i+1 {
            if 10000 * i + 5000 * j + 1000 * (n-i-j) == y {
                println!("{} {} {}", i, j, (n-i-j));
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
