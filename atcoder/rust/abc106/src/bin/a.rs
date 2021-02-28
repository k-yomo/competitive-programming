#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", a * b - (a + b - 1));
}
