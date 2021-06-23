#![allow(unused_imports)]
use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut cur_i = i;
        while cur_i <= n {
            ans += cur_i;
            cur_i += i;
        }
    }

    println!("{}", ans);
}
