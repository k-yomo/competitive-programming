#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut island_map = HashMap::new();
    ab.iter().filter(|(a, _)| *a == 1).for_each(|(_, b)| { island_map.insert(*b, true); });
    for (a, b) in ab {
        if *island_map.entry(a).or_default() && b == n || a == n && *island_map.entry(b).or_default() {
            println!("POSSIBLE");
            return;
        }
    }

    println!("IMPOSSIBLE");
}
