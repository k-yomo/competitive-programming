#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        m: usize,
        x: usize,
        a: [usize; m],
    }

    let cost = min(
        a.iter().filter(|&&i| i > x ).count(), 
        a.iter().filter(|&&i| i < x).count()
    );
    println!("{}", cost);
}
