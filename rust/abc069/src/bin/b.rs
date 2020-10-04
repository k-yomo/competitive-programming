#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        s: Chars,
    }

    println!("{}{}{}", s[0], s.len()-2, s.last().unwrap());
}
