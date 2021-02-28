#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use petgraph::unionfind::UnionFind;

fn main() { 
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut friend_conns = vec![vec![]; n];
    for (a, b) in ab {
        friend_conns[a-1].push(b-1);
        friend_conns[b-1].push(a-1);
    }

    for user in 0..n {
        let mut fof_map = HashMap::new();
        for &friend in friend_conns[user].iter() {
            for &fof in friend_conns[friend].iter() {
                if fof != user && !friend_conns[user].contains(&fof) {
                    fof_map.insert(fof.clone(), true);
                }
            }
        }
        println!("{:?}", fof_map.keys().count());
    }
}
