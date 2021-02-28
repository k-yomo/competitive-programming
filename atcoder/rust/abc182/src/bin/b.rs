#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max = 0;
    let mut max_count = 0;
    (2..1001).for_each(|i| {
        let count = a.iter().filter(|&&x| x % i == 0).count();
        if count > max_count {
            max_count = count;
            max = i;
        }

    });
    println!("{}", max);
}
