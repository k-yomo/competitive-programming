#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use proconio::marker::Chars;
use superslice::*;

fn main() {
    input! {
        (n, k): (usize, usize),
        d: [u32; k],
    }


    for i in n..99999 {
        let digits = i.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>();
        if digits.iter().all(|digit| !d.contains(&digit)) {
            println!("{}",i);
            return
        }
    }
}
