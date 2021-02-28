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
        (h, w, n): (usize, usize, usize),
        sections: [Chars; h],
    }

    let mut start = (0, 0);
    for (y, x) in iproduct!(0..h, 0..w) {
        if sections[y][x] == 'S' { start = (y, x) }
    }

    let mut total_time_min = 0;
    for i in 1..n + 1 {
        let mut visited = vec![vec![false; w]; h];
        let mut queue = VecDeque::new();
        queue.push_back((0, start));

        while queue.len() > 0 {
            let (distance, (y, x)) = queue.pop_front().unwrap();
            if y >= h || x >= w || visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            if sections[y][x] == 'X' {
                continue;
            }
            if sections[y][x] == std::char::from_digit(i as u32, 10).unwrap() {
                total_time_min += distance;
                start = (y, x);
                break;
            }
            queue.push_back((distance + 1, (y + 1, x)));
            queue.push_back((distance + 1, (y, x + 1)));
            if y > 0 { queue.push_back((distance + 1, (y - 1, x))); }
            if x > 0 { queue.push_back((distance + 1, (y, x - 1))); }
        }
    }

    println!("{}", total_time_min);
}
