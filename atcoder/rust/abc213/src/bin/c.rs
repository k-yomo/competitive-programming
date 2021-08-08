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
        h: usize, w: usize, n: usize,
        mut ab: [(usize, usize); n],
    }

    let mut row_exist_map = HashMap::new();
    let mut column_exist_map = HashMap::new();
    for (a, b) in ab.iter() {
        row_exist_map.insert(a, 0);
        column_exist_map.insert(b, 0);
    }

    let mut count = 0;
    for key in row_exist_map.clone().keys().sorted() {
        *row_exist_map.entry(*key).or_default() += count;
        count += 1;
    }
    count = 0;
    for key in column_exist_map.clone().keys().sorted() {
        *column_exist_map.entry(*key).or_default() += count;
        count += 1;
    }

    for (a, b) in ab.iter() {
        println!(
            "{} {}",
            row_exist_map.get(&a).unwrap() + 1,
            column_exist_map.get(&b).unwrap() + 1
        );
    }
}
