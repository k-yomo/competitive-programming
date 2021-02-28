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
    input! {
        s: usize,
        p: usize,
    }

    let mut i: usize = 1;
    loop {
        if &i * (s - &i) == p {
            println!("Yes");
            return
        }
        if &i * (s - &i) > p {
            break
        }
        i += 1;
    }
    println!("No");
}

