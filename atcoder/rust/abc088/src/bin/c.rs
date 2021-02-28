#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        c: [[i64; 3]; 3],
    }

    // a1 + b1, a1 + b2, a1 + b3
    // a2 + b1, a2 + b2, a2 + b3
    // a3 + b1, a3 + b2, a3 + b3
    let b1 = c[0][0];
    let b2 = c[0][1];
    let b3 = c[0][2];
    let a2 = c[1][0] - b1;
    let a3 = c[2][0] - b1;
    if c[1][1] == a2 + b2 && c[1][2] == a2 + b3 && c[2][0] == a3 + b1 && c[2][1] == a3 + b2 && c[2][2] == a3 + b3 {
        println!("Yes");
        return;
    }

    println!("No");
}
