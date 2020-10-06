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

    for i in (0..n+1).rev() {
        let sqrt = (i as f64).sqrt();
        if sqrt.floor()*sqrt.floor() == (i as f64) {
            println!("{}", i);
            return
        }
    }
}
