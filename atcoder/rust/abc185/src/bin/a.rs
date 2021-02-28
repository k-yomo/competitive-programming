#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        (a1, a2, a3, a4): (usize, usize, usize, usize),
    }

    println!("{}", min(min(a1,a2),min(a3,a4)));
}
