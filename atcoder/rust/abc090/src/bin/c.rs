#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        (n, m): (usize, usize),
    }
    if n == 1 && m == 1 {
        println!("1");
        return;
    }
    if n == 1 || m == 1 {
        println!("{}", n * m - 2);
        return;
    }
    println!("{}", n * m - (2 * (m + n) - 4));
}
