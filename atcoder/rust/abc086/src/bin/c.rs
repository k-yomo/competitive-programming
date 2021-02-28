#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools::__std_iter::once;

fn main() {
    input! {
        n: usize,
        txy: [(i64, i64, i64); n],
    }

    let mut cur_txy = (0, 0, 0);
    for (t, x, y) in txy {
        let need_time = (x - cur_txy.1).abs() + (y - cur_txy.2).abs();
        if (t - cur_txy.0) < need_time ||  (t - cur_txy.0 - need_time) % 2 != 0 {
            println!("No");
            return
        }
        cur_txy = (t, x, y);
    }
    println!("Yes");
}
