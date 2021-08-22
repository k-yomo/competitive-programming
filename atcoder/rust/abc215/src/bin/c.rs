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
        mut s: Chars, k: usize,
    }

    s.sort();

    let combinations = (0..s.len()).permutations(s.len());
    let mut exist_map = HashMap::new();
    let mut count = 0;
    for comb in combinations {
        let x = comb.iter().map(|&i| s[i]).collect::<String>();
        if exist_map.contains_key(&(x.clone())) {
            continue;
        }
        count += 1;
        if count == k {
            return println!("{}", x);
        }
        exist_map.insert(x, true);
    }
}
