#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        f: [[i64; 10]; n],
        p: [[i64; 11]; n],
    }

    let mut max_profit = std::i64::MIN;
    for bit_flag in 1..(1 << 10) {
        let mut both_open_counts: Vec<usize> = vec![0; n];
        for i in 0..10 {
            if bit_flag & (1 << i) == 0 {
                continue;
            }
            for (si, shop) in f.iter().enumerate() {
                if shop[i] == 1 {
                    both_open_counts[si] += 1;
                }
            }
        }
        let mut profit = 0_i64;
        for (si, both_open_count) in both_open_counts.iter().enumerate() {
            profit += p[si][*both_open_count];
        }
        ;
        max_profit = std::cmp::max(
            max_profit,
            both_open_counts.iter().enumerate().fold(0, |profit, (i, both_open_count)| profit + p[i][*both_open_count]),
        );
    }

    println!("{}", max_profit);
}
