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
        (h, w): (usize, usize),
        c: [Chars; h],
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (y, x) in iproduct!(0..h, 0..w) {
        if c[y][x] == 's' { start = (y, x) }
        if c[y][x] == 'g' { goal = (y, x) }
    }

    let mut visited = vec![vec![std::usize::MAX; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while queue.len() > 0 {
        let (cost, (y, x)) = queue.pop_front().unwrap();
        if cost > 2 || visited[y][x] < std::usize::MAX {
            continue;
        }
        visited[y][x] = cost;
        if c[y][x] == '#' {
            if y < h - 1 { queue.push_back((cost + 1, (y + 1, x))); }
            if x < w - 1 { queue.push_back((cost + 1, (y, x + 1))); }
            if y > 0 { queue.push_back((cost + 1, (y - 1, x))); }
            if x > 0 { queue.push_back((cost + 1, (y, x - 1))); }
        } else {
            if y < h - 1 { queue.push_front((cost, (y + 1, x))); }
            if x < w - 1 { queue.push_front((cost, (y, x + 1))); }
            if y > 0 { queue.push_front((cost, (y - 1, x))); }
            if x > 0 { queue.push_front((cost, (y, x - 1))); }
        }
    }

    println!("{}", if visited[goal.0][goal.1] <= 2 { "YES" } else { "NO" });
}
