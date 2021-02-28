#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;
use whiteread::parse_string;

fn main() { 
    input! {
        n: Chars,
    }

    let mut min_removed_count = n.len();
    for bit_flag in 0..(1 << n.len()) {
        let mut num_chars = vec![];
        for i in 0..n.len() {
            if bit_flag & 1 << i != 0 {
                num_chars.push(n[i])
            }
        }
        if num_chars.is_empty() {
            continue
        }
        if parse_string::<usize>(&num_chars.iter().collect::<String>()).unwrap() % 3 == 0 {
            let removed_count = n.len() - num_chars.len();
            if removed_count < min_removed_count {
                min_removed_count = removed_count
            }
        }
    }
    println!("{}", if min_removed_count == n.len() { -1 } else { min_removed_count as i64 } );
}
