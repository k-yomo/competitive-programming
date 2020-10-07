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
        c: usize,
        x: usize,
    }

    let mut count = 0;
    for five_hundred_yen_num in 0..a+1 {
        for one_hundred_yen_num in 0..b+1 {
            for fifty_yen_num in 0..c+1 {
                if five_hundred_yen_num * 500 + one_hundred_yen_num * 100 + fifty_yen_num * 50 == x {
                    count+= 1;
                }
            }
        }
    }

    println!("{}", count);
}
