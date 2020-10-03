#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    for i in a..b+1 {
        if i < a+k || i > b-k {
            println!("{}", i)
        }
    }
}
