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
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut comb_count = 0;
    for mut comb in (2..n + 1).permutations(n-1) {
        let mut perm = vec![1];
        perm.append(&mut comb);
        let is_valid = perm.windows(2).all(|w| {
            for (a, b) in ab.iter() {
                if a == &w[0] && b == &w[1] || a == &w[1] && b == &w[0] {
                    return true
                }
            }
            false
        });
        if is_valid {
            comb_count += 1;
        }
    }

    println!("{}", comb_count);
}
