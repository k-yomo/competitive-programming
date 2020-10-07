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
        mut a: [usize; n],
    }
    
    let mut alice_score = 0;
    let mut bob_score = 0;
    a.sort();
    for (i, score) in a.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice_score += score;
        } else {
            bob_score += score;
        }
    }

    println!("{}", alice_score - bob_score);
}
