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
        students: [(i64, i64); n],
        checkpoints: [(i64, i64); m],
    }

    for (a, b) in students {
        let mut min_len = 1000000000;
        let mut index = 0;
        for (i, (c, d)) in checkpoints.iter().enumerate() {
            let len = (a - c).abs() + (b - d).abs();
            if len < min_len {
                min_len = len;
                index = i;
            }
        }
        println!("{}", index + 1);
    }
}
