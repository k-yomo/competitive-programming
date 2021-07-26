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
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }

    let mut difficulty_count_map = HashMap::new();
    for i in d {
        *difficulty_count_map.entry(i).or_insert(0) += 1;
    }

    for i in t {
        let count = difficulty_count_map.entry(i).or_insert(0);
        if *count == 0 {
            return println!("NO");
        }
        *count -= 1;
    }

    println!("YES")
}
