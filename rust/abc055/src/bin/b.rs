#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

const MOD: u64 = 1000000007;
 
fn main() { 
    input! {
        n: u64,        
    }

    let mut power: u64 = 1;
    for i in 1..n+1 {
        power = power * i % MOD;
    }
    println!("{}", power % MOD);
}
