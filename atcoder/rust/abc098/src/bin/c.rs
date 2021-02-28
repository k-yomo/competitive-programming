#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let w_acc = once(0).chain(s.iter().map(|c| if *c == 'W' { 1 } else { 0 })).cumsum().collect::<Vec<usize>>();
    let e_acc = once(0).chain(s.iter().map(|c| if *c == 'E' { 1 } else { 0 })).cumsum().collect::<Vec<usize>>();
    println!("{}", s.iter().enumerate().map(|(i, c)| w_acc[i] + (e_acc[e_acc.len()-1] - e_acc[i+1])).min().unwrap());
}

