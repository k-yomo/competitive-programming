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

    let mut ac = a.clone();
    ac.sort();
    let mut m1 = ac[n - 1];
    let mut m2 = ac[n - 2];
    for i in a {
        if i == m1 {
            println!("{}", m2)
        } else {
            println!("{}", m1)
        }
    }
}
