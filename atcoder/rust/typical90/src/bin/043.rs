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
        h: usize, w: usize,
        rs: usize, cs: usize,
        rt: usize, ct: usize,
        s: [Chars; h],
    }

    let mut visited = vec![vec![std::i64::MAX; w]; h];
    let mut queue: VecDeque<(i64, usize, (usize, usize))> = VecDeque::new();
    queue.push_back((-1, 4, (rs, cs)));

    while queue.len() > 0 {
        let (cost, direction, (y, x)) = queue.pop_front().unwrap();
        if y == 0 || x == 0 || y > h || x > w {
            continue;
        }
        if s[y - 1][x - 1] == '#' {
            continue;
        }
        if visited[y - 1][x - 1] <= cost {
            continue;
        }

        visited[y - 1][x - 1] = cost;
        for (i, (&yd, &xd)) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)].iter().enumerate() {
            if direction == i {
                queue.push_front((cost, i, (y + yd as usize, x + yd as usize)));
            } else {
                queue.push_back((cost + 1, i, (y + yd as usize, x + yd as usize)));
            }
        }
    }

    println!("{}", visited[rt - 1][ct - 1])
}
