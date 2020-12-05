#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut p_i = 0;
    let mut q_i = 0;
    for (i, comb) in (1..n + 1).permutations(n).enumerate() {
        if comb.eq(&p) {
            p_i = i;
        }
        if comb.eq(&q) {
            q_i = i;
        }
    }
    println!("{}", (p_i as i64 - q_i as i64).abs());
}
