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
        k: [usize; n-1],
    }

    print!("{}", k[0]);
    for i in 0..k.len() {
        if i < k.len() - 1 && k[i + 1] < k[i] {
            print!(" {}", k[i + 1]);
        } else {
            print!(" {}", k[i]);
        }
    }
}
