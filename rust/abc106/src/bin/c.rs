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
       s: Chars,
       k: usize
    }
    for (i, c) in s.iter().enumerate() {
        if c != &'1' {
            println!("{}", c);
            return
        }
        if i+1 == k {
            println!("1");
            return
        }
    }
}
