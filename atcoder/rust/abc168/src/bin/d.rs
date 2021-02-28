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

    let mut d = vec![usize::max_value(); n];
    d[1] = 0;

    let mut g = vec![vec![false; n]; n];
    for (a, b) in ab {
        g[a - 1][b - 1] = true;
        g[b - 1][a - 1] = true;
    }

    let mut visited = vec![false; n];
    let mut ans = vec![0; n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while !que.is_empty() {
        let (cur, from) = que.pop_front().unwrap();
        if visited[cur] {
            continue;
        }
        ans[cur] = from;
        visited[cur] = true;
        for (i, e) in g[cur].iter().enumerate() {
            if visited[i] {
                continue;
            }
            if g[cur][i] {
                que.push_back((i, cur));
            }
        }
    }

    println!("Yes");
    for i in ans[1..].iter() {
        println!("{}", i + 1);
    }
}
