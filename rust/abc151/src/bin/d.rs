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

    let mut max_count = 0;
    let mut sg = vec![];
    for (y1, x1) in iproduct!(0..h, 0..w) {
        if a[y1][x1] == '#' {
            continue
        }
        for (y2, x2) in iproduct!(0..h, 0..w) {
            if (y1 == y2 && x1 == x2) || a[y2][x2] == '#' {
                continue
            }
            sg.push(((y1, x1), (y2, x2)))
        }
    }

    let mut max_dist = 0;
    for (s, g) in sg {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; w]; h];
        queue.push_back((0, s));
        while queue.len() > 0 {
            let (dist, (y, x)) = queue.pop_front().unwrap();
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            if a[y][x] == '#' {
                continue;
            }

            if y == g.0 && x == g.1 {
                if dist > max_dist {
                    max_dist = dist
                }
                break
            }

            if y < h - 1 { queue.push_back((dist + 1, (y + 1, x))); }
            if x < w - 1 { queue.push_back((dist + 1, (y, x + 1))); }
            if y > 0 { queue.push_back((dist + 1, (y - 1, x))); }
            if x > 0 { queue.push_back((dist + 1, (y, x - 1))); }
        }
    }

    println!("{}", max_dist);
}
