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
        mut n: usize,
    }

    if n % 2 != 0 {
        return;
    }

    let mut parentheses = vec![];
    let mut queue = vec![];
    queue.push((vec!['('], 1));
    while queue.len() > 0 {
        let (mut p, unclosed_count) = queue.pop().unwrap();
        if p.len() == n {
            parentheses.push(p);
            continue;
        }
        if unclosed_count == n - p.len() {
            for _ in 0..unclosed_count {
                p.push(')');
            }
            parentheses.push(p);
        } else {
            let mut a = p.clone();
            a.push('(');
            queue.push((a, unclosed_count + 1));
            if unclosed_count > 0 {
                let mut b = p.clone();
                b.push(')');
                queue.push((b, unclosed_count - 1));
            }
        }
    }

    for p in parentheses.iter().rev() {
        println!("{}", p.iter().collect::<String>());
    }
}
