#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use std::{cmp::Reverse, collections::BinaryHeap};
use std::iter::Map;

fn main() {
    input! {
        n: usize, q: usize,
        ab: [(usize, usize); n-1],
        cd: [(usize, usize); q],
    }


    let mut queue = VecDeque::new();
    let mut colors = vec![-1; n];
    colors[0] = 0;
    queue.push_back(0);

    let mut paths = vec![vec![]; n];
    for (a, b) in ab {
        paths[a-1].push(b-1);
        paths[b-1].push(a-1);
    }

    loop {
        if queue.is_empty() {
            break
        }
        let town = queue.pop_front().unwrap();
        for &i in paths[town].iter() {
            if colors[i] == -1 {
                colors[i] = 1 - colors[town];
                queue.push_back(i)
            }
        }
    }

    for (c, d) in cd {
        if colors[c-1] == colors[d-1] {
            println!("Town");
        } else {
            println!("Road")
        }
    }
}
