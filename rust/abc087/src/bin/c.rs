#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;

fn main() { 
    input! {
        n: usize,
        a1: [usize; n],
        a2: [usize; n],
    }

    let a1_acc = a1.iter().cumsum().collect::<Vec<usize>>();
    let a2_acc = once(0).chain(a2).cumsum().collect::<Vec<usize>>();

    println!("{}", (0..n).map(|i| { a1_acc[i] + (a2_acc[n] - a2_acc[i]) }).max().unwrap());
}
