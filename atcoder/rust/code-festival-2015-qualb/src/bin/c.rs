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
        n: usize, m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    if n < m {
        return println!("NO");
    }

    for (i, &people_num) in b.iter().rev().enumerate() {
        if a[n-1-i] < people_num {
            return println!("NO");
        }
    }

    println!("YES");
}
