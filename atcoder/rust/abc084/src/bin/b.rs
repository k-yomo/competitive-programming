#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize,
        b: usize,
        s: String,        
    }

    let split = s.split("-").collect::<Vec<&str>>();
    if split.len() != 2 || split[0].len() != a || split[1].len() != b {
        println!("No");
        return
    }
    println!("Yes");
}
