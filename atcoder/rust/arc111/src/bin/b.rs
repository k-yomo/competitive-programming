#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use petgraph::unionfind::UnionFind;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut ab:[(usize,usize); n]
    }

    let mut uf = Dsu::new(400001);
    let mut edges_count = vec![0; 400001];
    for (a, b) in ab {
        let a_root = uf.leader(a);
        let b_root = uf.leader(b);
        uf.merge(a, b);
        let root = uf.leader(a);
        edges_count[root] = 1 + edges_count[a_root] + edges_count[b_root]
    }

    let mut count = 0;
    for group in uf.groups() {
        let v = group.len();
        let edge_count = edges_count[uf.leader(group[0])];
        count += if v - 1 == edge_count { v - 1 } else { v };
    }

    println!("{}", count);
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