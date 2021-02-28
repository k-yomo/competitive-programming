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
        s: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((0, (0, 0)));

    let mut white_count = 0;
    while queue.len() > 0 {
        let (distance, (y, x)) = queue.pop_front().unwrap();
        if y >= h || x >= w || visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if s[y][x] == '#' {
            continue;
        }
        if y == h - 1 && x == w - 1 {
            white_count = distance + 1;
            break
        }
        queue.push_back((distance + 1, (y + 1, x)));
        queue.push_back((distance + 1, (y, x + 1)));
        if y > 0 { queue.push_back((distance + 1, (y - 1, x))); }
        if x > 0 { queue.push_back((distance + 1, (y, x - 1))); }
    }
    if white_count == 0 {
        println!("-1");
        return
    }

    println!("{}", iproduct!(0..h, 0..w).filter(|&(y, x)| s[y][x] != '#').count() - white_count);
}
