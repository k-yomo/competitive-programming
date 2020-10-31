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
        x: [i64; n],
    }

    let (mut manhattan_distance, mut euclid_distance, mut chebyshev_distance) = (0, 0, 0);
    for i in x {
        let i_abs = i.abs();
        manhattan_distance += i_abs;
        euclid_distance += i_abs * i_abs;
        chebyshev_distance = std::cmp::max(i_abs, chebyshev_distance);
    }

    println!("{}", manhattan_distance);
    println!("{}", (euclid_distance as f64).sqrt());
    println!("{}", chebyshev_distance);
}
