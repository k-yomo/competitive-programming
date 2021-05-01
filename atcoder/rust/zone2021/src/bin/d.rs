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

#[macro_export]
macro_rules! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
#[macro_export]
macro_rules! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[macro_export]
macro_rules! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }

fn main() {
    input! {
        s: String,
    }

    let mut t: VecDeque<char> = VecDeque::new();
    let mut is_reverse = false;
    for c in s.chars() {
        if c == 'R' {
            is_reverse = !is_reverse;
        } else if is_reverse {
            match t.pop_front() {
                Option::Some(val) => {
                    if val != c {
                        t.push_front(val);
                        t.push_front(c);
                    }
                }
                Option::None => {
                    t.push_front(c);
                }
            }
        } else {
            match t.pop_back() {
                Option::Some(val) => {
                    if val != c {
                        t.push_back(val);
                        t.push_back(c);
                    }
                }
                Option::None => {
                    t.push_back(c);
                }
            }
        }
    }

    if is_reverse {
        t = t.iter().rev().map(|x| *x).collect::<VecDeque<char>>();
    }

    println!("{}", Vec::from(t).iter().collect::<String>())
}
