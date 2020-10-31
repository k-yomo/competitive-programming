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

    let mut divisors = divisor(n);
    divisors.sort();
    for i in divisors {
        println!("{}", i);
    }
}


fn divisor(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in (1..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors
}