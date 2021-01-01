#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        (h, w): (usize, usize),
        c: (i64, i64),
        d: (usize, usize),
        s: [Chars; h],
    }

    let mut visited = vec![vec![std::usize::MAX; w]; h];
    let mut queue: VecDeque<(usize, (i64, i64))> = VecDeque::new();
    queue.push_back((0, (c.0 - 1, c.1 - 1)));

    while queue.len() > 0 {
        let (cost, (y, x)) = queue.pop_front().unwrap();
        if !(0 <= y && y < h as i64 && 0 <= x && x < w as i64) { continue; }

        let (uy, ux) = (y as usize, x as usize);
        if visited[uy][ux] < std::usize::MAX {
            continue;
        }
        visited[uy][ux] = cost;
        if s[uy][ux] == '#' {
            continue;
        }
        queue.push_front((cost, (y + 1, x)));
        queue.push_front((cost, (y, x + 1)));
        queue.push_front((cost, (y - 1, x)));
        queue.push_front((cost, (y, x - 1)));
        for (y_diff, x_diff) in iproduct!(0..3, 0..3) {
            let (y_diff, x_diff) = (y_diff as i64, x_diff as i64);
            if y_diff == 0 && x_diff == 0 { continue; }
            if y_diff + x_diff == 1 { continue; }
            queue.push_back((cost + 1, (y + y_diff, x)));
            queue.push_back((cost + 1, (y + y_diff, x + x_diff)));
            queue.push_back((cost + 1, (y, x + x_diff)));
            queue.push_back((cost + 1, (y - y_diff, x + x_diff)));
            queue.push_back((cost + 1, (y - y_diff, x)));
            queue.push_back((cost + 1, (y - y_diff, x - x_diff)));
            queue.push_back((cost + 1, (y, x - x_diff)));
            queue.push_back((cost + 1, (y + y_diff, x - x_diff)));
        }
    }

    println!("{}", if visited[d.0 - 1][d.1 - 1] == std::usize::MAX { -1 } else { visited[d.0 - 1][d.1 - 1] as i64 });
}
