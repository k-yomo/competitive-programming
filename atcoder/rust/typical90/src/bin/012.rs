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

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let Q = read();

    let mut grid = vec![vec![false; w]; h];
    let mut dsu = Dsu::new(h * w);
    for _ in 0..Q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let q = line
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        if q[0] == 1 {
            let (y, x) = (q[1] - 1, q[2] - 1);
            grid[y][x] = true;
            if y > 0 && grid[y - 1][x] {
                dsu.merge(y * w + x, (y - 1) * w + x);
            }
            if y < h - 1 && grid[y + 1][x] {
                dsu.merge(y * w + x, (y + 1) * w + x);
            }
            if x > 0 && grid[y][x - 1] {
                dsu.merge(y * w + x, y * w + x - 1);
            }
            if x < w - 1 && grid[y][x + 1] {
                dsu.merge(y * w + x, y * w + x + 1);
            }
        } else {
            if !grid[q[1] - 1][q[2] - 1] || !grid[q[3] - 1][q[4] - 1] {
                println!("No");
                continue;
            }
            println!(
                "{}",
                if dsu.leader((q[1] - 1) * w + (q[2] - 1))
                    == dsu.leader((q[3] - 1) * w + (q[4] - 1))
                {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/dsu.rs
pub struct Dsu {
    n: usize,
    // root node: -1 * component size
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

impl Dsu {
    // 0 <= size <= 10^8 is constrained.
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }
    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsu_works() {
        let mut d = Dsu::new(4);
        d.merge(0, 1);
        assert_eq!(d.same(0, 1), true);
        d.merge(1, 2);
        assert_eq!(d.same(0, 2), true);
        assert_eq!(d.size(0), 3);
        assert_eq!(d.same(0, 3), false);
        assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
    }
}
