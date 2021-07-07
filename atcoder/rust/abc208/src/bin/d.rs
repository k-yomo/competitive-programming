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
        n: usize, m: usize,
        abc: [(usize, usize, usize); m],
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}
impl From<char> for Dir {
    fn from(ch: char) -> Self {
        use Dir::{D, L, R, U};
        match ch {
            'U' => U,
            'D' => D,
            'L' => L,
            'R' => R,
            _ => unreachable!(),
        }
    }
}
impl From<(isize, isize)> for Dir {
    fn from((dy, dx): (isize, isize)) -> Self {
        use Dir::{D, L, R, U};
        match (dy, dx) {
            (-1, 0) => U,
            (1, 0) => D,
            (0, -1) => L,
            (0, 1) => R,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: u64,
    pub dir: Dir,
}

#[allow(clippy::many_single_char_names)]
pub fn dijkstra(
    g: &[Vec<Edge>],
    s: usize,
    calc_cost: impl Fn(&Edge, u64) -> Option<u64>,
) -> (Vec<Option<u64>>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = g.len();
    let mut dist = vec![None; n];
    let mut q = BinaryHeap::new();
    let mut prev = vec![None; n];
    dist[s] = Some(0);
    q.push((Reverse(0), s));
    while let Some((Reverse(d), v)) = q.pop() {
        if dist[v].map_or(false, |min| d > min) {
            continue;
        }
        for e in &g[v] {
            if let Some(cost) = calc_cost(e, d) {
                let next_d = d + cost;
                if dist[e.to].map_or(true, |cur_d| next_d < cur_d) {
                    dist[e.to] = Some(next_d);
                    prev[e.to] = Some(v);
                    q.push((Reverse(next_d), e.to));
                }
            }
        }
    }
    (dist, prev)
}

pub struct Around<'a> {
    y: usize,
    x: usize,
    y_range: std::ops::Range<usize>,
    x_range: std::ops::Range<usize>,
    directions: &'a [(isize, isize)],
    dir_idx: usize,
}

pub fn around<'a>(y: usize, x: usize) -> Around<'a> {
    Around {
        y,
        x,
        y_range: 0..std::usize::MAX,
        x_range: 0..std::usize::MAX,
        directions: &[],
        dir_idx: 0,
    }
}

impl<'a> Around<'a> {
    pub fn y_range(self, y_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            y_range: half_open_range(y_rng),
            ..self
        }
    }
    pub fn x_range(self, x_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            x_range: half_open_range(x_rng),
            ..self
        }
    }
    pub fn directions(self, dirs: &'a [(isize, isize)]) -> Self {
        Self {
            directions: dirs,
            ..self
        }
    }
}

impl<'a> Iterator for Around<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        fn dest(u: usize, i: isize) -> Option<usize> {
            if i.is_positive() {
                u.checked_add(i as usize)
            } else {
                u.checked_sub((-i) as usize)
            }
        }
        while let Some(&(dy, dx)) = self.directions.get(self.dir_idx) {
            self.dir_idx += 1;
            if let Some(ny) = dest(self.y, dy) {
                if let Some(nx) = dest(self.x, dx) {
                    if self.y_range.contains(&self.y)
                        && self.x_range.contains(&self.x)
                        && self.y_range.contains(&ny)
                        && self.x_range.contains(&nx)
                    {
                        return Some((ny, nx));
                    }
                }
            }
        }
        None
    }
}

fn half_open_range(rng: impl std::ops::RangeBounds<usize>) -> std::ops::Range<usize> {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    let start = match rng.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    };
    let end = match rng.end_bound() {
        Included(&e) => e + 1,
        Excluded(&e) => e,
        Unbounded => std::usize::MAX,
    };
    start..end
}

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
