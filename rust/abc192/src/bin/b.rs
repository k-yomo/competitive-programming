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
        s: Chars,
    }

    for (i, c) in s.iter().enumerate() {
        if (i % 2 == 0) == c.is_uppercase() {
            return println!("No");
        }
    }

    println!("Yes")
}
