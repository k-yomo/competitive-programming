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
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }

    println!("{}", (sy * gx + gy * sx) / (gy + sy));
}
