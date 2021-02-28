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
    }

    f(n, "".to_string());
}

fn f(rest: usize, s: String) {
    if rest == 0 {
        println!("{}", s);
    } else {
        for c in vec!["a", "b", "c"].iter() {
            f(&rest - 1, s.clone() + c);
        }
    }
}
