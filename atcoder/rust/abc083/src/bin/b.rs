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
       a: u32,
       b: u32, 
    }

    let mut sum = 0;
    for i in 1..n+1 {
        let v = i.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>();
        if a <= v && v <= b {
            sum += i;
        }
    }

    println!("{}", sum);
}
