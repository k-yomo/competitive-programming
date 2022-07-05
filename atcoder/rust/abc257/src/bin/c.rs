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

#[macro_export]
macro_rules! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    }

    let mut children = vec![];
    let mut adults = vec![];
    for (i, flag) in s.iter().enumerate() {
        match flag {
            &'0' => children.push(w[i]),
            &'1' => adults.push(w[i]),
            _ => {},
        }
    }

    children.sort();
    adults.sort();

    let mut max_correct_count = 0;
    for (i, size) in adults.iter().enumerate() {
        chmax!(max_correct_count, (adults.len() - i - 1) + children.lower_bound(size) + 1);
    }
    if adults.len() == 0 {
        max_correct_count = children.len()
    }
    println!("{}", max_correct_count);
}

