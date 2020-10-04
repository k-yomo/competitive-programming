#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        s: String,
    }

    println!("{}", s.rfind('Z').unwrap() - s.find('A').unwrap() + 1);
}
