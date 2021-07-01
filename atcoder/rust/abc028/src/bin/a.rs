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
        n: usize,
    }

    if n <= 59 {
        println!("Bad")
    } else if n >= 60 && n <= 89 {
        println!("Good")
    } else if n >= 90 && n <= 99 {
        println!("Great")
    } else {
        println!("Perfect")
    }
}
