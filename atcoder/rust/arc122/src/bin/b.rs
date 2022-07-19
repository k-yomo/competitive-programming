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
        n: usize,
        a: [f64; n],
    }

    let mut l = 0.0;
    let mut r = 1000000000.0;
    while (r - l) > 0.000001 {
        let ml = l + (r - l) / 3.0;
        let mr = r - (r - l) / 3.0;
        let ml_score = calc_score(&a, ml);
        let mr_score = calc_score(&a, mr);
        if ml_score > mr_score {
            l = ml;
        } else {
            r = mr;
        }
    }

    println!("{}", calc_score(&a, l))
}

fn calc_score(a: &Vec<f64>, x: f64) -> f64 {
    return a.iter().map(|&ai| x + ai - if ai > 2.0 * x { 2.0 * x } else { ai }).sum::<f64>() / a.len() as f64;
}
