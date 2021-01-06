#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
    }
    
    let mut min_digit = n;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            let digit_len = std::cmp::max(i, n / i).to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>().len();
            min_digit = std::cmp::min(min_digit, digit_len);
        }
        i += 1;
    }
    println!("{}", min_digit);
}
