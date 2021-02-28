#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        (n, m, t): (i64, usize, i64),
        ab: [(i64, i64); m],
    }

    let mut battery = n;
    let mut cur_time = 0;
    for (a, b) in ab {
        battery -= a - cur_time;
        if battery <= 0 {
            break
        }
        battery += b-a;
        if battery > n {
            battery = n;
        }
        cur_time = b;
    }
    battery -= t-cur_time;

    println!("{}", if battery > 0 { "Yes" } else { "No" });
}
