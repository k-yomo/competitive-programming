#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        (n, k): (usize, usize),
        cards: [usize; n],
    }

    let mut num_map = HashMap::new();
    for perm in (0..n).permutations(k) {
        let mut num_strs = vec![];
        for &j in perm.iter() {
            num_strs.push(cards[j].to_string())
        }
        num_map.insert(num_strs.join(""), true);
    }
    println!("{}", num_map.keys().len());
}
