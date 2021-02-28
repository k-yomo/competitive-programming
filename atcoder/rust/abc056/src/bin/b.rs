#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        w: usize,
        a: usize,
        b: usize,
    }

if b+w < a {
   println!("{}", a-(b+w));
} else if b > a+w {
   println!("{}", b-(a+w));
} else {
   println!("{}", 0);
}
}
