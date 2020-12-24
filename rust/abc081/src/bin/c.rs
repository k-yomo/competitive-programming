#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }

    let mut num_map = HashMap::new();
    for num in a {
        let count: &mut usize = num_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut num_counts: Vec<&usize> = num_map.values().collect();
    if num_counts.len() <= k {
        println!("0");
        return;
    }

    num_counts.sort();
    println!("{}", num_counts[..(num_counts.len() - k)].iter().map(|&&num| num).sum::<usize>());
}
