#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        (n, mut k): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|x| x.0);

    for (a, b) in ab {
        if b < k {
            k -= b;
        } else {
            println!("{}", a);
            return;
        }
    }
}
