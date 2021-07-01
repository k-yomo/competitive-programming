#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
    }

    let mut char_counts = vec![0; 6];
    for c in s {
        let i = match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            _ => 6,
        };
        char_counts[i] += 1;
    }

    println!("{}", char_counts.iter().join(" "));
}
