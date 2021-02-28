#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", max(max(a+b, a-b), a*b));
}
