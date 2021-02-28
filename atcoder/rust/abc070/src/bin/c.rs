#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use num_integer::lcm;

fn main() { 
    input! {
        n: usize,
        t: [usize; n],
    }

    let mut ans = t[0];
    for &n in t[1..].iter() {
        ans = lcm(ans, n);
    }

    println!("{}", ans);
}
