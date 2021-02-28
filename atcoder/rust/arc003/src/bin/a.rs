#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: f64,
        r: Chars,
    }

    let mut total_gpa = 0.0;
    for gpa in r {
        total_gpa += match gpa {
            'A' => 4.0,
            'B' => 3.0,
            'C' => 2.0,
            'D' => 1.0,
            _ => 0.0,
        }
    }
    println!("{}", total_gpa / n);
}
