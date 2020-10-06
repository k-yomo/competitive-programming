#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        x: usize,
        y: usize,
        z: usize,        
    }

    println!("{}", (x-z) / (y+z));
}
