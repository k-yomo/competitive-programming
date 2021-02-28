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
        (r, c): (usize, usize),
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        maze: [Chars; r],
    }

    let mut visited = vec![vec![false; c]; r];
    let mut queue = VecDeque::new();
    queue.push_back((0, (sy - 1, sx - 1)));

    while queue.len() > 0 {
        let (distance, (y, x)) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if maze[y][x] == '#' {
            continue;
        }
        if y == gy - 1 && x == gx - 1 {
            println!("{}", distance);
            return;
        }
        queue.push_back((distance + 1, (y + 1, x)));
        queue.push_back((distance + 1, (y, x + 1)));
        queue.push_back((distance + 1, (y - 1, x)));
        queue.push_back((distance + 1, (y, x - 1)));
    }
}
