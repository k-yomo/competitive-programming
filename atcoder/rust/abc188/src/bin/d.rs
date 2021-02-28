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
        n: usize, C: i64,
        mut abc: [(i64, i64, i64); n],
    }

    let mut price_map: HashMap<i64, i64> = HashMap::new();
    for (a, b, c) in abc {
        let p = price_map.entry(a).or_insert(0);
        *p += c;
        let p = price_map.entry(b+1).or_insert(0);
        *p -= c;
    }
    let mut day_price_diff: Vec<(i64, i64)> = price_map.into_iter().map(|(d, p)| (d, p)).collect();
    day_price_diff.sort_by_key(|x| x.0);
    let mut cur_price = 0;
    let mut min_total = 0_i64;
    for (i, (d, p)) in day_price_diff[..(day_price_diff.len()-1)].iter().enumerate() {
        cur_price += p;
        min_total += (day_price_diff[i+1].0 - d) * std::cmp::min(C, cur_price);
    }


    println!("{}", min_total);
}
