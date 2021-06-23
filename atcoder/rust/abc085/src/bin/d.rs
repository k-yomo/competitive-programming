#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        n: usize, h: usize,
        mut ab: [(usize, usize); n],
    }

    let a_max = ab.iter().max_by_key(|x| x.0).unwrap().0;
    ab.sort_by_key(|x| x.1);

    let mut count = 0;
    let mut damage = 0;
    for (_, b) in ab.iter().rev() {
        if b > &a_max && damage < h {
            damage += b;
            count += 1;
        } else {
            break;
        }
    }
    if h > damage {
        count += (h - damage + a_max - 1) / a_max;
    }
    println!("{}", count);
}
