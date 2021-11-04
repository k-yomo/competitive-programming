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
        n: usize, q: usize,
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    }

    let mut shift_count = 0;
    for (t, x, y) in txy {
        match t {
            1 => {
                let x_i = if x <= shift_count { n + x - shift_count } else { x - shift_count } - 1;
                let y_i = if y <= shift_count { n + y - shift_count } else { y - shift_count } - 1;
                let tmp = a[x_i];
                a[x_i] = a[y_i];
                a[y_i] = tmp;
            }
            2 => {
                shift_count += 1;
                shift_count %= n;
            }
            3 => {
                let x_i = if x <= shift_count { n + x - shift_count } else { x - shift_count } - 1;
                println!("{}", a[x_i])
            }
            _ => {}
        }
    }
}
