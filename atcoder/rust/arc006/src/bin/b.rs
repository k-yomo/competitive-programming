#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
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
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

fn main() {
    let (n, l) = read_tuple!(usize, usize);

    let amida = read_vec(l, || {
        read::<String>()
            .chars()
            .skip(1)
            .step_by(2)
            .map(|c| c == '-')
            .collect_vec()
    });

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let goal_idx = line
        .chars()
        .step_by(2)
        .find_position(|&c| c == 'o')
        .unwrap()
        .0;

    let mut cur = (l - 1, goal_idx);
    let mut visited = vec![[false; 10]; l];
    loop {
        visited[cur.0][cur.1] = true;

        if cur.1 > 0 && amida[cur.0][cur.1 - 1] && !visited[cur.0][cur.1 - 1] {
            cur.1 -= 1;
        } else if cur.1 < n - 1 && amida[cur.0][cur.1] && !visited[cur.0][cur.1 + 1] {
            cur.1 += 1;
        } else if cur.0 > 0 {
            cur.0 -= 1;
        } else {
            break;
        }
    }
    println!("{}", cur.1 + 1);
}
