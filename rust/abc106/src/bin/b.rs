#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
        n: usize,
    }
    let mut count = 0;
    for i in 1..n+1 {
        let mut divisors = 0;
        for j in 1..i+1 {
            if j % 2 == 1 && i % j == 0 {
                divisors += 1;
            }
        }
        if divisors == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
