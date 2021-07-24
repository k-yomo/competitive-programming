#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::prelude::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use superslice::*;

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

const Mod: u128 = 1000000007;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(Edge { to: b - 1, cost: 1 });
        graph[b - 1].push(Edge { to: a - 1, cost: 1 });
    }

    let mut queue = BinaryHeap::new();
    // (cost, vertex)
    queue.push(Rev((0, 0)));

    let mut shortest_path_num = vec![0; n];
    shortest_path_num[0] = 1;

    let mut min_costs = vec![std::usize::MAX; n];
    min_costs[0] = 0;

    while !queue.is_empty() {
        let Rev((min_cost, v)) = queue.pop().unwrap();
        if min_costs[v] < min_cost {
            continue;
        }
        for e in &graph[v] {
            if min_costs[e.to] > min_costs[v] + e.cost {
                min_costs[e.to] = min_costs[v] + e.cost;
                queue.push(Rev((min_costs[e.to], e.to)));

                shortest_path_num[e.to] = shortest_path_num[v];
            } else if min_costs[e.to] == min_costs[v] + e.cost {
                shortest_path_num[e.to] = (shortest_path_num[e.to] + shortest_path_num[v]) % Mod;
            }
        }
    }

    println!("{}", shortest_path_num[n - 1] % Mod);
}
