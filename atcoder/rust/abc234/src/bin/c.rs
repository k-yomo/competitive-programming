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
        k: usize,
    }

    println!("{}", format!("{:b}", k).chars().map(|c| if c == '1' { '2' } else { c }).collect::<String>());
}
