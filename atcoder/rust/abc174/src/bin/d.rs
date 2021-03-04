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
        n: usize,
        c: Chars,
    }

    let a_list = c.into_iter().map(|color| if color == 'R' { 1 } else { 0 }).collect::<Vec<usize>>();
    let r_count = a_list.iter().sum::<usize>();

    println!("{}", r_count - a_list[0..r_count].iter().sum::<usize>())
}
