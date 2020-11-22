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
        x: usize,
        s: Chars,
    }

    let mut score = x;
    for c in s {
        if c == 'x' {
            if score > 0 {
                score -= 1;
            }
        } else {
            score += 1;
        }
    }
    println!("{}", score);
}
