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
        ab: [(usize, usize); m],
    }

    let mut city_road_map: HashMap<usize, HashMap<usize, bool>> = HashMap::new();
    for (a, b) in ab {
        city_road_map.entry(b - 1).or_default().insert(a - 1, true);
    }

    let mut count = 0;
    for i in 0..n {
        let mut visited = vec![false; n];
        let mut queue = vec![];
        queue.push(i);
        visited[i] = true;
        while queue.len() > 0 {
            let cur = queue.pop().unwrap();
            count += 1;
            for (k, _) in city_road_map.entry(cur).or_default() {
                if !visited[*k] {
                    queue.push(*k);
                    visited[*k] = true;
                }
            }
        }
    }
    println!("{}", count)
}
