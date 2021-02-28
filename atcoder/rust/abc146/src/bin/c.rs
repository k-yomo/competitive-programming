#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    let (mut ok, mut ng) = (1, 10_usize.pow(9) + 1);
    let mut mid = (ok + ng) / 2;
    while (ng-ok) > 1 {
        if x >= a * mid + b * mid.to_string().len() {
            ok = mid;
        } else {
            ng = mid;
        }
        mid = (ok + ng) / 2;
    }
    if x < (a * mid + b * mid.to_string().len() as usize) {
        mid = 0;
    }
    println!("{}", mid);
}

