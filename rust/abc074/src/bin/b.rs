#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: i64,
        k: i64,
        mut x: [i64; n as usize],
    }

    let mut total_cost = 0;
    for ball_pos in x {
        let cost_from_k = (ball_pos - k).abs();
        total_cost += if cost_from_k < ball_pos { cost_from_k * 2 } else { ball_pos * 2 };
    }
    println!("{}", total_cost)
}
