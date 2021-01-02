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
        n: usize,
        ab: [(usize, usize); n],
        q: usize,
        mut tex: [(usize, usize, usize); q],
    }


    let mut nodes = vec![0; n];
    tex.sort_by(|(a, b)| a.1.cmp(&b.1));

    let uf = UnionFind::new(n);
    let mut text_i = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        uf.union(a, b);
    }

    for (t, e, x) in tex {
        let (mut from, mut except);
        if  {
            (from, except) = ab[e - 1];
        } else {
            (except, from) = ab[e - 1];
        }
    }

   nodes.iter().for_each(|node|  println!("{}", node));
}

