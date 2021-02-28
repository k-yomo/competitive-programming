#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut sorted_x = x.clone();
    sorted_x.sort();
    let med1 = sorted_x[n / 2 - 1];
    let med2 = sorted_x[n / 2];
    for num in x {
        println!("{}", if num <= med1 { med2 } else { med1 });
    }
}
