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
        n: usize,
        mut s: [String; n]
    }

    s = s
        .iter()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>();
    s.sort();

    for rev_word in s {
        println!("{}", rev_word.chars().rev().collect::<String>())
    }
}
