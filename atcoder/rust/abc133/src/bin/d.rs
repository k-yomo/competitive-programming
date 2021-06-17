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
    }

    let mut s = a.iter().sum::<usize>();
    let mut last_dam = s - 2 * (a[1..].iter().step_by(2).sum::<usize>());
    print!("{}", last_dam);
    for i in 0..n - 1 {
        last_dam = (a[i] - (last_dam / 2)) * 2;
        print!(" {}", last_dam)
    }
}
