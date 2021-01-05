#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        (n, m): (usize, usize),
        (x, y): (usize, usize),
        a: [usize; n],
        b: [usize; m],
    }

    let (mut ai, mut bi) = (0_usize, 0_usize);
    let mut roundtrip_count = 0;
    while ai < n && bi < m {
        bi = b.lower_bound(&(a[ai] + x));
        if bi >= m {
            break;
        }
        ai = a.lower_bound(&(b[bi] + y));
        roundtrip_count += 1;
    }

    println!("{}", roundtrip_count);
}
