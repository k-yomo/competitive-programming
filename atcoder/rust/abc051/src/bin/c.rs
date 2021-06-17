#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        sx: i64, sy: i64, tx: i64, ty: i64,
    }

    (1..=(ty - sy)).for_each(|i| print!("U"));
    (1..=(tx - sx)).for_each(|i| print!("R"));
    (1..=(ty - sy)).for_each(|i| print!("D"));
    (1..=(tx - sx)).for_each(|i| print!("L"));

    print!("L");
    (1..=(ty - sy) + 1).for_each(|i| print!("U"));
    (1..=(tx - sx) + 1).for_each(|i| print!("R"));
    print!("DR");
    (1..=(ty - sy) + 1).for_each(|i| print!("D"));
    (1..=(tx - sx) + 1).for_each(|i| print!("L"));
    print!("U");
}
