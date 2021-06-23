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
        na: usize, nb: usize,
        a: [usize; na],
        b: [usize; nb],
    }

    let mut num_map = HashMap::new();
    for i in a.iter().chain(b.iter()) {
        *num_map.entry(i).or_insert(0) += 1;
    }

    println!(
        "{}",
        num_map.iter().filter(|(_, &count)| count > 1).count() as f64
            / num_map.iter().filter(|(_, &count)| count > 0).count() as f64
    );
}
