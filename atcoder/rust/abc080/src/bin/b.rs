#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: String,        
    }

    if n.parse::<u32>().unwrap() % n.chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
