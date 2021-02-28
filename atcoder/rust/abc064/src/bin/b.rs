#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    println!("{}", a.last().unwrap() - a.first().unwrap());
}
