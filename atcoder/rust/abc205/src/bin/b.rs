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
        mut a: [usize; n],
    }

    a.sort();
    for i in 0..n {
        if i + 1 != a[i] {
            return println!("No");
        }
    }
    println!("Yes")
}
