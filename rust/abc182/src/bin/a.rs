#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", 2 * a + 100 - b);
}
