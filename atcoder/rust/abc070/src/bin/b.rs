#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: i64,        
        b: i64,        
        c: i64,        
        d: i64,        
    }

    println!("{}", max(0, min(b,d) - max(a,c)));
}
