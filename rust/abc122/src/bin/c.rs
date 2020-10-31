#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
       n: usize,
       q: usize,
       s: Chars,
       ranges: [(usize,usize); q]
    }

    let mut cumulative_sum = Vec::new();
    cumulative_sum.push(0);
    for i in 1..n {
        if s[i-1] == 'A' && s[i] == 'C' {
            cumulative_sum.push(cumulative_sum[i-1] + 1);
        } else {
            cumulative_sum.push(cumulative_sum[i-1]);
        }
    }

    let mut sum = 0;
    for (l,r) in ranges {
        println!("{}", cumulative_sum[r-1] - cumulative_sum[l-1]);
    }
}
