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
        ab: [(usize, usize); n],
        q: usize,
        mut tex: [(i64, usize, usize); q],
    }

    let mut edges = vec![vec![]; n];
    let mut counters = vec![0_i64; n];

    for (a, b) in ab.iter() {
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }

    for (t, e, x) in tex {
        if t == 1 {
            counters[0] += t;
            counters[ab[e].1 - 1] -= t;
        } else {
            counters[ab[e].1 - 1] += t;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut processed = vec![false; n];
    processed[0] = true;
    while queue.len() > 0 {
        let i = queue.pop_front().unwrap();
        for conn in &edges[i] {
            if processed[*conn] {
                continue
            }
            counters[*conn] += counters[i];
            processed[*conn] = true;
            queue.push_back(*conn);

        }
    }

    for counter in counters {
        println!("{}", counter);
    }
}
