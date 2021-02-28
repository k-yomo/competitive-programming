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
        (n, k, q) : (usize, usize, usize),
        a: [usize; q],
    }

    let mut answer_counts = vec![0; n];
    a.iter().for_each(|i|  answer_counts[i-1] += 1);
    (0..n).for_each(|i|  {
        if k > q - answer_counts[i] {
            println!("Yes")
        } else {
            println!("No")
        }
    });
}
