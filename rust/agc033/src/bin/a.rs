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
        a: [Chars; h],
    }

    let mut queue = VecDeque::new();
    let mut max_count = 0;
    for (y, x) in iproduct!(0..h, 0..w) {
        if (y < h - 1 && a[y + 1][x] == '#')
            || (y > 0 && a[y - 1][x] == '#')
            || (x < w - 1 && a[y][x + 1] == '#')
            || (x > 0 && a[y][x - 1] == '#')
        {
            queue.push_back((1, (y, x)));
        }
    }

    let mut visited = vec![vec![false; w]; h];
    while queue.len() > 0 {
        let (count, (y, x)) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if a[y][x] == '#' {
            continue;
        }

        max_count = std::cmp::max(max_count, count);

        if y < h - 1 { queue.push_back((count + 1, (y + 1, x))); }
        if x < w - 1 { queue.push_back((count + 1, (y, x + 1))); }
        if y > 0 { queue.push_back((count + 1, (y - 1, x))); }
        if x > 0 { queue.push_back((count + 1, (y, x - 1))); }
    }

    println!("{}", max_count);
}
