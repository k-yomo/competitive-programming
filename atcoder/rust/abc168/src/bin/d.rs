#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut depths = vec![-1; n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while !que.is_empty() {
        let (cur, from) = que.pop_front().unwrap();
        if depths[cur] >= 0 {
            continue;
        }
        depths[cur] = from as i32;
        for i in g[cur].iter() {
            if depths[*i] >= 0 {
                continue;
            }
            que.push_back((*i, cur));
        }
    }

    println!("Yes");
    for i in depths[1..].iter() {
        println!("{}", i + 1);
    }
}
