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
        t: [usize; n],
    }

    let mut min_time = t.iter().sum();
    for bit_flag in 0..(1 << n) {
        let (mut left_total, mut right_total) = (0, 0);
        for i in 0..t.len() {
            if bit_flag & (1 << i) != 0 {
                left_total += t[i];
            } else {
                right_total += t[i];
            }
        }
        let max = if left_total > right_total { left_total } else {right_total };
        if max < min_time {
            min_time = max;
        }
    }
    println!("{}", min_time);
}
