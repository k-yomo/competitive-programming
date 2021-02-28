#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use petgraph::unionfind::UnionFind;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut bridge_count = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        let mut uf = UnionFind::new(n);
        for (j, (a, b)) in ab.iter().enumerate() {
            if i != j {
                uf.union(a - 1, b - 1);
            }
        }
        if !uf.equiv(a - 1, b - 1) {
            bridge_count += 1;
        }
    }

    println!("{}", bridge_count);
}
