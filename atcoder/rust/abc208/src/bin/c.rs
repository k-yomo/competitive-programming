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
        n: usize, mut k: i128,
        a: [usize; n],
    }

    let mut a = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect::<Vec<(usize, usize)>>();
    a.sort_by_key(|x| x.1);

    let mut snack_counts: Vec<i128> = vec![0; n + 1];

    let to_all = k / (n as i128);
    snack_counts[0] += to_all;
    snack_counts[n] -= to_all;
    let to_some = (k as usize % n);
    snack_counts[0] += 1;
    snack_counts[to_some] -= 1;

    let mut people_snack_counts = vec![0; n];
    let mut cur = 0;
    for (i, &snack_count) in snack_counts[0..n].iter().enumerate() {
        people_snack_counts[a[i].0] = snack_count + cur;
        cur += snack_count;
    }

    people_snack_counts.iter().for_each(|x| println!("{}", x))
}
