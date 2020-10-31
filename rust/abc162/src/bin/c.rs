#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;
use num::integer::gcd;

fn main() { 
    input! {
        k: usize,
    }

    let mut gcd_sum = 0;
    for a in 1..k+1 {
        for b in 1..k+1 {
            for c in 1..k+1 {
                gcd_sum += gcd(gcd(a,b),c);
            }
        }
    }

    println!("{}", gcd_sum);
}
