#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();

    let mut sum = 0;
    let mut que = VecDeque::new();
    que.push_back(a[0]);
    for i in 1..n {
        sum += que.pop_front().unwrap();
        que.push_back(a[i]);
        que.push_back(a[i]);
    }

    println!("{}", sum);
}
