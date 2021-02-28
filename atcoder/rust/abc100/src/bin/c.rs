#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! { n: usize }

    println!("{}", (0..n).map(|_| {
        input! { mut a: usize }
        let mut mod_count = 0;
        while a % 2 == 0 {
            a /= 2;
            mod_count += 1;
        }
        mod_count
    } ).sum::<usize>());
}
