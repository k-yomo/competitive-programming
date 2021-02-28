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
        (n, q): (usize, usize),
        ab: [(usize, usize); n - 1],
        px: [(usize, usize); q],
    }

    let mut connections = vec![vec![]; n];
    let mut counters = vec![0_usize; n];

    for (a, b) in ab {
        connections[a - 1].push(b - 1);
        connections[b - 1].push(a - 1);
    }

    for (p, x) in px {
        counters[p - 1] += x;
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut processed = vec![false; n];
    processed[0] = true;
    while queue.len() > 0 {
        let i = queue.pop_front().unwrap();
        for conn in &connections[i] {
            if processed[*conn] {
                continue
            }
            counters[*conn] += counters[i];
            processed[*conn] = true;
            queue.push_back(*conn);

        }
    }

    for counter in counters {
        print!("{} ", counter);
    }
}
