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

    if s.len() == s.chars().collect::<HashSet<char>>().len() {
        println!("yes");
    } else {
        println!("no");
    }
}
