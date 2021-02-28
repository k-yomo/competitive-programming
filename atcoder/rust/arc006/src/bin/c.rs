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
        n: usize,
        w: [usize; n],
    }

    let mut rows = vec![std::usize::MAX];
    for b in w {
        let mut should_push = true;
        for i in 0..rows.len() {
            if b <= rows[i] {
                rows[i] = b;
                should_push = false;
                break
            }
        }
        if should_push {
            rows.push(b);
        }
    }

    println!("{}", rows.len());
}
