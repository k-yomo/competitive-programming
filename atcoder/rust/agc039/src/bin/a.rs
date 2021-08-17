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
        s: Chars,
        k: usize,
    }

    if s.len() == 1 {
        return println!("{}", k / 2);
    }

    let dup_counts = s
        .iter()
        .map(|c| (c, 1))
        .coalesce(|(prev_c, prev_dup), (cur_c, cur_dup)| {
            if prev_c == cur_c {
                Ok((prev_c, prev_dup + cur_dup))
            } else {
                Err(((prev_c, prev_dup), (cur_c, cur_dup)))
            }
        })
        .map(|(_, count)| count)
        .filter(|&count| count > 1)
        .collect::<Vec<usize>>();
    if dup_counts.len() == 1 {
        return println!("{}", s.len() * k / 2);
    } else {
        if s[0] == s[s.len() - 1] {
            let len = dup_counts.len();
            let mut dup_count = 0;
            dup_count += dup_counts[0] / 2;
            dup_count += dup_counts[len - 1] / 2;
            dup_count += (dup_counts[0] + dup_counts[len - 1]) / 2 * (k - 1);
            if len >= 3 {
                dup_count += dup_counts[1..(len - 1)]
                    .iter()
                    .map(|count| count / 2 * k)
                    .sum::<usize>();
            }
            println!("{}", dup_count)
        } else {
            println!(
                "{}",
                dup_counts.iter().map(|count| count / 2 * k).sum::<usize>()
            )
        }
    }
}
