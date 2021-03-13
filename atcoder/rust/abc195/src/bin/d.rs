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
     n: usize, m: usize, q: usize,
     mut wv: [(usize, usize); n],
     x: [usize; m],
     query: [(usize, usize); q]
    }

    wv.sort_by(|a, b| b.1.cmp(&a.1));
    let mut mx = x
        .iter()
        .enumerate()
        .map(|(i, w)| (i + 1, *w))
        .collect::<Vec<(usize, usize)>>();
    mx.sort_by(|a, b| a.1.cmp(&b.1));

    // 価値が一番高いものが、一番小さい重さの箱にマッチングしていけばよい
    for (l, r) in query {
        let mut sum = 0;
        let mut used: HashMap<usize, bool> = HashMap::new();
        for (w, v) in &wv {
            for (i, W) in mx.iter() {
                if l <= *i && *i <= r {
                    continue;
                }
                if !(*used.entry(*i).or_default()) && w <= W {
                    sum += v;
                    used.insert(*i, true);
                    break;
                }
            }
        }
        println!("{}", sum);
    }
}
