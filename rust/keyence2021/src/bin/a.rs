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
        a: [usize; n],
        b: [usize; n],
    }

    let mut a_max = 0;
    let mut a_actual_max = 0;
    let mut b_max = 0;
    let mut ab_max = 0;
    for i in 0..n {
        a_actual_max = std::cmp::max(a_actual_max, a[i]);
        if b_max <= b[i] {
            a_max = std::cmp::max(a_actual_max, a[i]);
            b_max = std::cmp::max(b_max, b[i]);
        }
        ab_max = std::cmp::max(ab_max, std::cmp::max(a_max * b_max, a_actual_max * b[i]));
        println!("{}", ab_max);
    }
}
