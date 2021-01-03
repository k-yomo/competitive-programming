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
        n: usize,
        mut b: [usize; n],
    }

    b.sort();
    let max_num = b.last().unwrap();
    let mut found_num_map = HashMap::new();
    let mut uniq_num_count = 0;
    for i in 0..n {
        let mut cur = b[i];
        if *found_num_map.entry(cur).or_default() {
            continue
        }
        uniq_num_count += 1;
        while cur < *max_num {
            cur *= 2;
            found_num_map.insert(cur, true);
        }
    }

    println!("{}", uniq_num_count);
}
