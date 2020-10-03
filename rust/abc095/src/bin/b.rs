#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,        
        x: usize,        
        m: [usize; n], 
    }

    let restElements = x - m.iter().sum::<usize>();
    let total = n + (restElements / m.iter().min().unwrap());
    println!("{}", total);
}
