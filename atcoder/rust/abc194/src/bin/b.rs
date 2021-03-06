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
        mut ab: [(usize, usize); n],
    }

    let mut a = ab
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.0))
        .collect::<Vec<(usize, usize)>>();
    a.sort_by_key(|x| x.1);
    let mut b = ab
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.1))
        .collect::<Vec<(usize, usize)>>();
    b.sort_by_key(|x| x.1);

    if a[0].0 != b[0].0 {
        println!("{}", std::cmp::max(a[0].1, b[0].1))
    } else {
        println!(
            "{}",
            std::cmp::min(
                std::cmp::min(a[0].1 + b[0].1, std::cmp::max(a[0].1, b[1].1)),
                std::cmp::max(a[1].1, b[0].1),
            )
        );
    }
}
