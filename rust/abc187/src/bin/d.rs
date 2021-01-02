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
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|a, b| (a.0 * 2 + a.1).cmp(&(b.0 * 2 + b.1)));

    let mut aoki_vote_acc = vec![0; n+1];
    for (i, (a, _)) in ab.iter().enumerate() {
        aoki_vote_acc[i+1] = aoki_vote_acc[i] + *a;
    }
    aoki_vote_acc.reverse();

    let mut takahashi_vote = 0;
    for (i, (a, b)) in ab.iter().rev().enumerate() {
        takahashi_vote += a + b;
        if takahashi_vote > aoki_vote_acc[i + 1] {
            println!("{}", i + 1);
            return
        }
    }
}
