#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::Itertools;
 
fn main() { 
    input! {
        k: i64,
        s: i64,
    }

    let mut count = 0;
    for i in 0..k+1 {
        for j in 0..k+1 {
            let res = s - i - j;
            if res >= 0 && res <= k {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
