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
        coordinates: [(i64, i64); n],
    }

    for perm in (0..n).combinations(3) {
        let (x1, y1) = coordinates[perm[0] as usize];
        let (x2, y2) = coordinates[perm[1] as usize];
        let (x3, y3) = coordinates[perm[2] as usize];
        if (x1 - x3) * (y2 - y3) == (x2 - x3) * (y1 - y3) {
            println!("Yes");
            return
        }
    }
    println!("No");
}
