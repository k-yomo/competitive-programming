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

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        queries: [(usize, i64); q],
    }

    let mut moved_count: i64 = 0;
    for (op, x) in queries {
        match op {
            1 => moved_count = (moved_count + x) % n as i64,
            2 => {
                let i = if x > moved_count {
                    (x - 1 - moved_count) as usize
                } else {
                    n - (x - 1 - moved_count).abs() as usize
                };
                println!("{}", s[i])
            }
            _ => {}
        }
    }
}
