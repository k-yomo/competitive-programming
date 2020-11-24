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
        k: usize,
        t: [[usize; n]; n],
    }

    let mut count = 0;
    for perm in (1..n).into_iter().permutations(n - 1) {
        let mut total_time = t[0][perm[0]];
        for (i, &cur_town) in perm[0..perm.len() - 1].iter().enumerate() {
            total_time += t[cur_town][perm[i + 1]];
        }
        total_time += t[*perm.last().unwrap()][0];
        if total_time == k {
            count += 1;
        }
    }
    println!("{}", count);
}
