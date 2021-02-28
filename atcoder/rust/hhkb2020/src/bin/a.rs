#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        s: char,
        t: char,
    }

    if s == 'Y' {
        println!("{}", t.to_uppercase());
    } else {
        println!("{}", t);
    }
}
