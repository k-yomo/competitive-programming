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
        n: usize, k: usize,
        t: [[usize; k]; n],
    }

    println!(
        "{}",
        if dfs(n, k, &t, 0, 0) {
            "Found"
        } else {
            "Nothing"
        }
    )
}

fn dfs(n: usize, k: usize, t: &Vec<Vec<usize>>, h: usize, v: usize) -> bool {
    if h == n {
        return v == 0;
    }
    for i in 0..k {
        if dfs(n, k, t, h + 1, v ^ t[h][i]) {
            return true;
        }
    }
    return false;
}
