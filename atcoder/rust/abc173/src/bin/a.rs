#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,        
    }

    for i in 1..11 {
        if i * 1000 >= n {
            println!("{}", i * 1000 - n);
            return
        }
    }
}
