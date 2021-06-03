#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }

    let mut even: HashMap<usize, usize> = HashMap::new();
    let mut odd: HashMap<usize, usize> = HashMap::new();
    for (i, &num) in v.iter().enumerate() {
        if i % 2 == 0 {
            *even.entry(num).or_insert(0) += 1;
        } else {
            *odd.entry(num).or_insert(0) += 1;
        }
    }
    let (even_max, even_max_count) = even.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let (odd_max, odd_max_count) = odd.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    if even_max == odd_max {
        let (_, even_second_max_count) = even
            .iter()
            .filter(|a| a.0 != even_max)
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or((&0, &0));
        let (_, odd_second_max_count) = odd
            .iter()
            .filter(|b| b.0 != odd_max)
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or((&0, &0));
        println!(
            "{}",
            n - std::cmp::max(
                even_max_count + odd_second_max_count,
                even_second_max_count + odd_max_count
            )
        );
    } else {
        println!("{}", n - even_max_count - odd_max_count);
    }
}
