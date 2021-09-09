#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars; h],
    }

    let mut max_length = 0_i64;
    for (y, x) in iproduct!(0..h, 0..w) {
        if c[y][x] != '#' {
            max_length = std::cmp::max(max_length, dfs(&c, &vec![vec![false; w]; h], (y, x), (y, x), 0).unwrap_or(0))
        }
    }

    println!("{}", if max_length >= 3 { max_length } else { -1 })
}

fn dfs(graph: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, start: (usize, usize), cur: (usize, usize), length: i64) -> Option<i64> {
    let mut cloned_visited = visited.clone();
    if cloned_visited[cur.0][cur.1] {
        if cur.0 == start.0 && cur.1 == start.1 {
            return Some(length)
        }
        return None
    }
    cloned_visited[cur.0][cur.1] = true;

    let mut max_length = 0;
    if cur.0 > 0 && graph[cur.0-1][cur.1] != '#' {
        max_length = std::cmp::max(max_length, dfs(graph, &cloned_visited, start, (cur.0 -1, cur.1), length + 1).unwrap_or(0))
    }
    if cur.1 < graph[0].len() - 1 && graph[cur.0][cur.1 + 1] != '#' {
        max_length = std::cmp::max(max_length, dfs(graph, &cloned_visited, start, (cur.0, cur.1 + 1), length + 1).unwrap_or(0))
    }
    if cur.0 < graph.len() - 1 && graph[cur.0+1][cur.1] != '#' {
        max_length =  std::cmp::max(max_length, dfs(graph, &cloned_visited, start, (cur.0 + 1, cur.1), length + 1).unwrap_or(0))
    }
    if cur.1 > 0 && graph[cur.0][cur.1 - 1] != '#' {
        max_length = std::cmp::max(max_length, dfs(graph, &cloned_visited, start, (cur.0, cur.1 - 1), length + 1).unwrap_or(0))
    }
    return Some(max_length)
}
