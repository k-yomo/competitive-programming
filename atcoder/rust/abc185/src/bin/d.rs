#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use num_integer::{gcd, lcm};
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut a: [usize; m],
    }

    if m == n {
        println!("0");
        return;
    }
    if m == 0 {
        println!("1");
        return;
    }

    a.push(0);
    a.push(n + 1);
    a.sort();

    let k = a.windows(2).map(|a| a[1] - a[0] - 1).filter(|&a| a > 0).min().unwrap();
    println!("{}", a.windows(2).map(|a| { (a[1] - a[0] - 1 + k - 1) / k }).sum::<usize>());
}
