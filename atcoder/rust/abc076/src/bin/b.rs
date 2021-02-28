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
        k: usize,        
    }

    let mut num = 1;
    for _ in 0..n {
        if num * 2 < num + k {
            num *= 2;
        } else {
            num += k;
        }
    }
    
    println!("{}", num);
}
