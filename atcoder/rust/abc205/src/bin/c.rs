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
        a: i128,
        b: i128,
        c: i128,
    }

    println!(
        "{}",
        if a == b {
            "="
        } else if c % 2 == 0 {
            if a.abs() == b.abs() {
                "="
            } else if a.abs() > b.abs() {
                ">"
            } else {
                "<"
            }
        } else {
            if a > b {
                ">"
            } else {
                "<"
            }
        }
    );
}
