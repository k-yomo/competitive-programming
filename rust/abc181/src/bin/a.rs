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
    println!("{}", if n % 2 == 0 { "White" } else { "Black" });
}
