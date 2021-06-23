#![allow(unused_imports)]
use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        a: usize,
    }

    println!("{}", a + a.pow(2) + a.pow(3));
}
