#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        d: [usize; n],
    }

    let uniq_diameters: HashSet<usize> = d.into_iter().collect();
    println!("{}", uniq_diameters.len())
}
