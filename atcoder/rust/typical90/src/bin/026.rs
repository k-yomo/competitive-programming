#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut paths = vec![vec![]; n];
    for (a, b) in ab {
        paths[a - 1].push(b - 1);
        paths[b - 1].push(a - 1);
    }

    let mut nodes: Vec<usize> = vec![2; n];
    dfs(&mut nodes, &paths, 0, 0);
    let i = if nodes.iter().filter(|&&x| x == 0).count() >= n / 2 {
        0
    } else {
        1
    };
    nodes
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == i)
        .take(n / 2)
        .for_each(|(i, _)| println!("{}", i + 1))
}

fn dfs(nodes: &mut Vec<usize>, paths: &Vec<Vec<usize>>, pos: usize, cur: usize) {
    nodes[pos] = cur;
    for &i in paths[pos].iter() {
        if nodes[i] == 2 {
            dfs(nodes, paths, i, 1 - cur)
        }
    }
}
