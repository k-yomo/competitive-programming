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
        (t, n): (usize, usize),
        a: [usize; n],
        m: usize,
        b: [usize; m],
    }

    let mut ai = 0_usize;
    for arrive_at in b {
        loop {
            if ai > n - 1 || a[ai] > arrive_at {
                println!("no");
                return
            }
            if arrive_at <= a[ai] + t {
                ai += 1;
                break
            }
            ai += 1;
        }
    }

    println!("yes");
}
