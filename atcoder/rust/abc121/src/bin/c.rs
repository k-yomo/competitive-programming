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
        (n, mut m): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|(a1), (a2)| a1.cmp(a2));

    let mut price = 0;
    for (a, b) in ab {
        if m < b {
            price += m * a;
            break;
        }
        price += b * a;
        m -= b;
    }

    println!("{}", price);
}
