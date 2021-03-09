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
        a: [i64; n],
    }

    let a_odd_acc = once(0)
        .chain(
            a.iter()
                .enumerate()
                .map(|(i, x)| if i % 2 == 0 { *x } else { 0 }),
        )
        .cumsum()
        .collect::<Vec<i64>>();
    let a_even_acc = once(0)
        .chain(
            a.iter()
                .enumerate()
                .map(|(i, x)| if i % 2 != 0 { *x } else { 0 }),
        )
        .cumsum()
        .collect::<Vec<i64>>();

    let mut takahashi_max = -100;
    for i in 1..=n {
        let mut aoki_score = -100;
        let mut takahashi_score = -100;
        for j in (1..=n).rev() {
            if i == j {
                continue;
            }
            let l = std::cmp::min(i, j);
            let r = std::cmp::max(i, j);
            let mut a_score = 0;
            let mut t_score = 0;
            if l % 2 == 0 {
                t_score = a_even_acc[r] - a_even_acc[l - 1];
                a_score = a_odd_acc[r] - a_odd_acc[l - 1];
            } else {
                t_score = a_odd_acc[r] - a_odd_acc[l - 1];
                a_score = a_even_acc[r] - a_even_acc[l - 1];
            };
            if aoki_score <= a_score {
                aoki_score = a_score;
                takahashi_score = t_score;
            }
        }
        takahashi_max = std::cmp::max(takahashi_max, takahashi_score);
    }

    println!("{}", takahashi_max);
}
