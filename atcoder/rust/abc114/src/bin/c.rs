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
        n: usize,
    }
    let count = dfs(3, n) + dfs(5, n) + dfs(7, n);
    println!("{}", count);
}

fn dfs(i: usize, n: usize) -> usize {
    if i > n || i > 1000000000 {
        return 0;
    }
    let i_str = i.to_string();
    let c = if i_str.contains('3') && i_str.contains('5') && i_str.contains('7') {
        1
    } else {
        0
    };
    return dfs(i * 10 + 3, n) + dfs(i * 10 + 5, n) + dfs(i * 10 + 7, n) + c;
}
