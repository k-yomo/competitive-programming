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
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut cur_town = 1;
    let mut count = 0;
    let mut visited = HashMap::new();
    while *visited.get(&cur_town).unwrap_or(&0) == 0 {
        visited.insert(cur_town, count);
        cur_town = a[cur_town - 1];
        count += 1;
        if count == k {
            return println!("{}", cur_town);
        }
    }

    let k_rest = (k - count) % (count - *visited.get(&cur_town).unwrap());
    for _ in 0..k_rest {
        cur_town = a[cur_town - 1];
    }

    println!("{}", cur_town);
}
