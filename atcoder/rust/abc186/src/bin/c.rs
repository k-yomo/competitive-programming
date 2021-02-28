#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        n: u32,
    }
    println!("{}", (1..n+1).filter(|i| !(i.to_string().contains("7") || format!("{:o}", i).contains("7"))).count());
}
