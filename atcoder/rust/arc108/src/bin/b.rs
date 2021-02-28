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
        s: Chars,
    }

    let mut t = vec![];
    let mut fox_count = 0;
    for c in s {
        t.push(c);
        if t.len() < 3 {
            continue
        }
        if t[t.len()-3..].iter().collect::<String>() == "fox" {
            fox_count += 1;
            t = t[..t.len()-3].to_vec()
        }
    }
    println!("{}", n - fox_count * 3);
}


